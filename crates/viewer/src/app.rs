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

use editor_core::appearance::Rgba8;
use eframe::egui;
use egui_tiles::{ContainerKind, EditAction, Tile, TileId, Tiles, Tree, UiResponse};
use pncad::geom_core::Tol;

use crate::camera::{Camera, CameraError};
use crate::display::DisplayView;
use crate::drafts::{Drafts, ProfileDoors};
use crate::evalseam::FitService;
#[cfg(not(target_family = "wasm"))]
use crate::evalseam::ThreadEvaluator;
use crate::frame::{self, RankedVerdict};
use crate::gpu::{DEPTH_BITS, ViewportRenderer};
use crate::idpass::{IdChannel, IdQueryLog};
use crate::input::InputMap;
use crate::marks;
use crate::parts::PartChooser;
use crate::pickcache::{self, IndexSeam, PickCache};
use crate::pickindex::{EdgeNamesRefused, PickIndex, PictureKey};
use crate::platform;
use crate::prefs::{self, Prefs, PrefsStore};
use crate::scene::{self, DisplayTolerance, SceneError, SceneMesh};
use crate::session::{DocSession, Refusal, SaveChoice, SessionOp, Step};
use crate::sketch::{self, PreviewError, ProfilePreview};
use crate::theme::{Polarity, Theme};
use crate::tools::Tools;

// The one re-export this module carries: `tests/panel_display.rs`
// reaches the field-writing value by the `viewer::app::FieldWriting`
// path to assert the unit/tick pair on it.
pub use crate::forms::FieldWriting;

#[cfg(all(any(test, feature = "test-support"), not(target_family = "wasm")))]
#[doc(hidden)]
pub mod drive;

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

/// What the New… form's commit button says: what it makes, so it reads
/// apart from every other commit on screen.
const NEW_DOCUMENT_COMMIT: &str = "Create document";

/// The tab name of the container holding the feature tree and the
/// properties — the document's model, as against the View pane's
/// display settings.
const MODEL_TAB_TITLE: &str = "Model";

/// A container's tab title, in words.
///
/// A tab title is prose a person reads, so the layout vocabulary is
/// spelled here rather than taken from `ContainerKind`'s `Debug`,
/// which would show a user a kind added upstream as a type
/// identifier.
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
/// The cause is said in its OWN words: the layer that raised the
/// indeterminacy names it, and this one contributes only the noun it
/// is talking about. Its node is said by `by`, which speaks for the
/// document the cause's ids are spelled in. Named rather than composed
/// inside the render pass so the wording has one home and can be
/// asserted on.
pub fn indeterminate_wording(
    noun: &str,
    cause: &editor_core::ResolveIndeterminate,
    by: pncad::document::Speaker<'_>,
) -> String {
    format!(
        "this {noun} cannot be resolved right now: {}",
        pncad::document::Said(cause, by)
    )
}

/// The most of the Features/Properties stack the feature tree is
/// auto-given: past this, the tree scrolls in its own half rather than
/// crowding the properties out.
const FEATURES_SHARE_CAP: f32 = 0.5;

/// Points of breathing room under the feature tree when its height is
/// what sizes the tile.
const FEATURES_SLACK: f32 = 8.0;

/// What the toolbar says while a canceled run leaves an older picture
/// on screen (`frame::Progress::Canceled`).
const CANCELED_LINE: &str = "canceled — showing an older result";

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
/// **The one place the tone-to-chrome mapping is made**: anything drawn
/// in a [`frame::Tone`] is drawn through this, directly or through
/// `widgets::message_toned`, so what `Advisory` looks like is changed
/// here or nowhere. Its callers are whatever `rg 'toned\('` lists; they
/// are not listed here, because a list is what falls behind.
///
/// Everything wearing [`crate::theme::Theme::actionable`] says its own
/// words, so this decides salience and never meaning.
pub(crate) fn toned(text: impl Into<String>, theme: &Theme, tone: frame::Tone) -> egui::RichText {
    let text = egui::RichText::new(text);
    match tone {
        frame::Tone::Advisory => text.weak(),
        frame::Tone::Actionable => text.color(chrome(theme.actionable)),
    }
}

/// **Draw a control whose refusal decides both halves of it**: live
/// exactly while `blocked` is `None`, and out of it carrying that
/// refusal's own words as the reason it cannot be used. Answers
/// whether it was clicked.
///
/// The composition, for a BUTTON, of the rule *a control a reader
/// cannot use owes the sentence a click would have been answered with*
/// (`crates/viewer/README.md`). Its callers are whatever
/// `rg 'refusable_button\('` lists, and they spelled the same three
/// lines until they were one call. The caller keeps what a click
/// MEANS, because they push different things. A control of another
/// shape obeys the rule in its own body: the Properties pane's unit
/// picker is a combo over up to three components and composes the
/// same two hooks itself (`ViewerBehavior::slot_unit_ui`).
pub(crate) fn refusable_button(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    blocked: Option<&Refusal>,
) -> bool {
    let button = ui.add_enabled(blocked.is_none(), egui::Button::new(label));
    match blocked {
        Some(refusal) => button.on_disabled_hover_text(refusal.to_string()).clicked(),
        None => button.clicked(),
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
pub(crate) fn apply_polarity(ctx: &egui::Context, polarity: Polarity) {
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
/// boxes and every placed body in the feature tree carried a fourth.
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
/// The world badge: marks a body the world places, in the feature tree
/// — see [`GLYPH_REMOVE`].
pub(crate) const GLYPH_WORLD: &str = "»";

/// One docked pane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pane {
    /// The 3D viewport.
    Viewport,
    /// The feature tree over the evaluation's result DAG.
    Features,
    /// The selected node's (or variable's) properties.
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
    /// document with a failed placement does not re-tessellate every
    /// healthy copy on every repainted frame.
    picks: PickCache,
    /// Where the id pass leaves its reply.
    id_answer: IdChannel,
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
    /// Held rather than announced: the viewport draws nothing under it,
    /// so "the picture is missing and the rebuild will not run" is true
    /// on every frame until a rebuild lands, and a badge is a read of
    /// exactly this.
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
    /// **What the viewport found on the last frame**, read by the
    /// per-read badges in [`crate::frame`]. One frame behind for the
    /// reason above, and unlike the fault above never held past the
    /// frame that made it ([`ViewportReads`]).
    viewport_reads: ViewportReads,
    /// What the incoming document is owed when its scene lands, and the
    /// budget's verdict about it ([`DocumentDerived`]).
    derived: DocumentDerived,
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
    /// **The modal tools, at most one open** — every tool
    /// [`crate::tools::ToolKind`] names, as one value, with
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
    /// The last thing that went wrong, kept so a refused operation is
    /// visible instead of silently dropped — with what it is ABOUT, so
    /// the next event about that subject can retire it
    /// ([`crate::frame::Subject`]).
    status: Option<frame::Message>,
    /// The tool that was open at the last batch — what a change of the
    /// open tool is measured against, so the line's
    /// [`frame::Subject::Tool`] messages retire with the tool they are
    /// about ([`frame::tool_status`]).
    line_tool: Option<crate::tools::ToolKind>,
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
    /// once at startup ([`platform::chooser_backend`]); the Open, Save
    /// and Save As controls read it every frame.
    chooser: platform::ChooserBackend,
    /// **The choice a refused save left standing** ([`SaveChoice`]):
    /// drawn on the toolbar until the person picks one of its two
    /// saves, dismisses it, or a later save, open or new document
    /// answers it.
    save_choice: Option<SaveChoice>,
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
    /// The OS entropy source refused the starting document's id
    /// (`WorkspaceError::RandomnessUnavailable`).
    Identity(pncad::workspace::WorkspaceError),
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
            Self::Identity(error) => {
                write!(f, "the starting document has no id: {error}")
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

/// The coincidence door's seam this build runs on — [`evaluator`]'s
/// choice again, for its reason.
///
/// # Errors
///
/// [`StartupError::Worker`] if the OS refuses the thread; the wasm arm
/// is infallible. Every landing's checks would otherwise read their
/// door pending forever.
fn prover() -> Result<Box<dyn crate::evalseam::DoorService>, StartupError> {
    #[cfg(not(target_family = "wasm"))]
    {
        Ok(Box::new(
            crate::evalseam::ThreadProver::spawn().map_err(StartupError::Worker)?,
        ))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Box::new(crate::evalseam::InlineProver::new()))
    }
}

/// The staged seam: [`prover`]'s shape over a staged operation's run.
///
/// # Errors
///
/// [`StartupError::Worker`] if the OS refuses the thread; the wasm arm
/// is infallible. Every staged verdict would otherwise read checking
/// forever.
fn stager() -> Result<Box<dyn crate::evalseam::StagedService>, StartupError> {
    #[cfg(not(target_family = "wasm"))]
    {
        Ok(Box::new(
            crate::evalseam::ThreadStager::spawn().map_err(StartupError::Worker)?,
        ))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Box::new(crate::evalseam::InlineStager::new()))
    }
}

/// **What the viewer owes the document under the session when its
/// first scene lands, and what the budget said about it.**
///
/// **One value, because replacing the document invalidates all of it
/// at once** — the session's own [`crate::session`] `Derived` move,
/// one layer up. [`DocumentDerived::owed`] is the one spelling of "a
/// document has just arrived": the value [`ViewerApp::document_replaced`]
/// installs for every operation [`SessionOp::replaces_the_document`]
/// names, and the base [`DocumentDerived::at_startup`] is built from. A
/// field added here is reset by being declared.
///
/// **A field belongs here when a replacement must reset it and nothing
/// else would.** A value keyed on the landed generation, δ or picture
/// revision is retired by the next landing, a per-frame value is
/// rewritten every frame, and session chrome must survive; none of
/// those belongs here. The drafts and the open tool hold ids of the
/// document alongside chrome, so each drops its own half in the same
/// door.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DocumentDerived {
    /// A camera fit is owed until a rebuilt scene with bounds lands.
    /// The document arrives asynchronously, so fitting at the door
    /// would frame the outgoing picture.
    fit_camera: bool,
    /// Whether the first landing with a body to price should have its
    /// δ CHOSEN by the triangle budget, rather than drawn at the δ
    /// already in force.
    ///
    /// Cleared the moment it is spent, or when the user sets δ. It is what makes the budget a
    /// DEFAULT: a document arrives, the budget picks a δ that draws it
    /// in a reasonable time, and from then on δ is whatever the user
    /// asked for. Anything stronger — clamping every rebuild — would
    /// make the View pane's δ field a control that does nothing on
    /// exactly the documents someone would want it for. The δ the last
    /// document was read at says nothing about this one: it is a
    /// length, and the new document may be a different size.
    fit_delta: bool,
    /// The budget's verdict, while the δ it chose is still the δ in
    /// force. `None` once the user has moved δ themselves
    /// ([`ViewerApp::set_delta`] clears it), because from there the
    /// number on screen is theirs and the badge would be claiming a
    /// choice it did not make.
    budget_delta: Option<crate::scene::FittedDelta>,
}

impl DocumentDerived {
    /// A document has just arrived: both fits owed, no verdict yet.
    fn owed() -> Self {
        Self {
            fit_camera: true,
            fit_delta: true,
            budget_delta: None,
        }
    }

    /// The startup document: [`Self::owed`] except the camera fit,
    /// which `pending_fit` takes on the first frame over the startup
    /// mesh already drawn. Owing it again at the first landing would
    /// re-frame a camera the user moved in between.
    fn at_startup() -> Self {
        Self {
            fit_camera: false,
            ..Self::owed()
        }
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
        Self::assemble_with(egui_ctx, tol, prefs_store())
    }

    /// [`Self::assemble`] over the preferences `store` it reads and
    /// writes.
    fn assemble_with(
        egui_ctx: &egui::Context,
        tol: Tol,
        store: Store,
    ) -> Result<Self, StartupError> {
        let delta = DisplayTolerance::new(scene::INITIAL_DELTA).map_err(StartupError::Scene)?;
        // The startup plate is a new document, so it mints its id as
        // `SessionOp::NewDocument` does; the scene fixture derives one.
        // The `Identity` arm is UNTESTED, and labelled as such:
        // `getrandom` has no injection seam.
        let id = pncad::workspace::random_document_id().map_err(StartupError::Identity)?;
        let (document, _root) = scene::plate_with_hole(tol).map_err(StartupError::Document)?;
        let document = document.under_identity(id);
        let mesh = scene::scene_of(&document, delta, tol).map_err(StartupError::Scene)?;
        // A provisional camera, because no pane has been laid out yet
        // and there is no aspect to fit against. It is provisional for
        // exactly one frame: `pending_fit` below makes the viewport fit
        // what it draws at its true aspect the first time it runs — the
        // fit a replaced document books when its scene lands, taken at
        // once here because the startup mesh is already drawn — so what
        // a user sees is never this.
        let camera = Camera::default();

        // Preferences, before anything is drawn. A refusal here is
        // never fatal: a viewer that would not open because its
        // colour scheme was unreadable would be trading the whole
        // product for a preference, so every arm ends in a message
        // and the defaults.
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
        let (notation, notation_notices) = saved.resolve_notation();
        notices.extend(
            [theme_notice, keys_notice]
                .into_iter()
                .flatten()
                .chain(notation_notices)
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

        // And the toolkit's input options: a wheel is a scroll whatever
        // modifier is held (`configure_input`'s doc says why).
        crate::pane::viewport::configure_input(egui_ctx);

        let mut session = DocSession::new(document, tol, evaluator()?)
            .with_door(prover()?)
            .with_stager(stager()?);
        session.set_notation(notation);

        Ok(Self {
            session,
            delta,
            scene: Arc::new(mesh),
            picks: PickCache::new(indexer()?),
            id_answer: IdChannel::new(),
            id_log: IdQueryLog::new(),
            revision: 1,
            scene_key: None,
            scene_display: None,
            scene_focus: BTreeSet::new(),
            scene_fault: None,
            projection_fault: None,
            viewport_reads: ViewportReads::default(),
            // The startup document's δ goes through the same budget an
            // opened one's does: it is small enough that the budget
            // will not move it, and a first picture that took a
            // different path from every later one is a difference
            // waiting to be a bug.
            derived: DocumentDerived::at_startup(),
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
            // Whatever the preferences file had to say, in the one
            // place this crate puts a thing that went wrong.
            status: frame::startup_notices(&notices),
            line_tool: None,
            notices: Vec::new(),
            chooser: platform::chooser_backend(),
            save_choice: None,
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
    /// ([`DocumentDerived::fit_delta`]) — so the typed number wins,
    /// silently, because nothing was taken away from anyone.
    ///
    /// A fit that REFUSED leaves δ alone: the document is one whose
    /// placements do not gather, or one that tessellates at NEITHER of the
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
                self.derived.budget_delta = fitted.requested_cost.map(|_| fitted);
            }
        }
    }

    /// **The one wait the toolbar names**, read from the session and
    /// the index seam ([`PickCache::index_seam`], which reads the fit
    /// the build waits on as well as the build).
    ///
    /// **The spinner follows the work, never the name**
    /// (`frame::Progress::Canceled`).
    fn progress(&self) -> Option<frame::Progress> {
        frame::progress(
            self.session.outstanding(),
            self.picks.index_seam(self.fit.as_ref()),
        )
    }

    /// Take whatever the seam finished and, if the picture is behind
    /// the document, rebuild it.
    ///
    /// The one place a document becomes a scene. Gated on the
    /// generation so a frame that changed nothing re-tessellates
    /// nothing.
    fn sync_scene(&mut self) {
        self.session.pump();
        // What the landings' prunes withdrew (DI5: a placement that pins
        // a free-moved space takes effect when its solve lands), said as
        // an operation's withdrawals are.
        let withdrawn = self.session.take_landed_withdrawals();
        self.notices
            .extend(withdrawn.iter().flat_map(frame::withdrawal_notices));
        // **The survival step, once per frame, for whichever tool is
        // open** — the obligation every tool's module docs put on its
        // consumer, discharged in the one place that holds them all.
        // Each notice already names its tool.
        let dropped = self
            .tools
            .reconcile(self.session.doc(), self.session.landed_pair());
        // A tool's survival drop is provoked by the document transition
        // it did not survive, so the act that accepts the next one is
        // what retires it; `frame::tool_notice` reads from the event
        // whether the pick is lost for good.
        let landed = self.session.landed_pair();
        self.notices.extend(
            dropped
                .iter()
                .map(|notice| frame::tool_notice(notice, landed)),
        );
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
        // Spent on the first landing with a body to price. A landing
        // with none (an empty new document, a product that refuses) has
        // no size to fit a δ to, so the budget waits for the edit that
        // gives the document one: until then the δ in force is the
        // outgoing document's, which says nothing about this one. A δ
        // the user types in the meantime disarms it
        // ([`ViewerApp::set_delta`]), so it stays a default.
        if self.derived.fit_delta
            && let Some(request) = self.session.fit_request(self.delta)
        {
            self.derived.fit_delta = false;
            self.fit.submit(request);
        }
        // **The index seam answers here.** A build that finished is
        // installed or refused now, before the currency check below
        // reads the cache — so an index that landed for the picture on
        // screen is used on the frame it arrives rather than the next
        // one. A refusal is held by the cache and badged, not
        // announced; a superseded answer is nothing to say
        // (`pickcache::IndexLanding::Stale` is what restart-without-cancel
        // produces, once per δ changed mid-build). A copy the index
        // could not draw is not a refusal: the index draws the rest and
        // holds the copy's reason (`PickIndex::undrawn`).
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
        // re-attempted every frame.
        //
        // **A held refusal draws nothing**, never the picture from
        // before it: that picture is of a run the index no longer
        // describes, and every pick against it is refused. The arms that
        // wait for a build (`Submitted`, `Indexing`) and the one with
        // nothing settled to build (`Nothing`) leave the viewport
        // drawing the mesh it already has — an older picture, which the
        // indexing indicator names and `pickcache::unindexed` refuses
        // picks against (a canceled run is the other, `CANCELED_LINE`).
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
            pickcache::CacheStep::Held => {
                // The index's badge says it; a scene fault about the
                // picture this replaces is no longer true of any.
                self.draw_nothing();
                self.scene_fault = None;
                return;
            }
            pickcache::CacheStep::Nothing
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
                // The key is set ONLY on success: a refused build
                // leaves the empty picture on screen with no key, so
                // nothing reads its ids as this index's.
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
                // A boundless scene is framed (the origin) but does not
                // spend the fit: the document's first body is what it
                // is owed for.
                if self.derived.fit_camera {
                    self.derived.fit_camera = self.scene.bounds().is_none();
                    self.pending_fit = true;
                }
                // The gather's own verdict is NOT written here: it is
                // a standing fact about the landed pair, not this
                // frame's news, and the status line carries news
                // (`frame`'s header). It badges beside the at-rest and
                // checks reads, off `frame::product_badge`, which is a
                // read of held state and so cannot be stale here or
                // erased by anything the rest of the frame does.
            }
            // Held, not announced: the fault is true of the empty
            // picture on screen until a rebuild succeeds — never of the
            // picture from before it.
            Err(error) => {
                self.draw_nothing();
                // Tried under these inputs: the next index, display or
                // focus change retries it, and an unchanged frame does not.
                self.scene_display = Some(display_revision);
                self.scene_focus = focus;
                self.scene_fault = Some(error);
            }
        }
    }

    /// Put the empty picture on screen, once: the scene under a pick
    /// index or a scene rebuild refused as a whole. No index minted its
    /// ids, so it carries no key.
    fn draw_nothing(&mut self) {
        if self.scene_key.is_none() && self.scene.positions().is_empty() {
            return;
        }
        self.scene = Arc::new(SceneMesh::nothing(self.delta));
        self.scene_key = None;
        self.scene_display = None;
        self.scene_focus = BTreeSet::new();
        self.revision = self.revision.wrapping_add(1);
    }

    /// Change δ to `delta`, rebuilding the picture from the evaluation
    /// already in hand — a display change is not a document change and
    /// re-runs no geometry above the tessellator.
    fn set_delta(&mut self, delta: DisplayTolerance) {
        self.delta = delta;
        // From here the number is the user's. The budget chose an
        // opening δ and has no further say — including no say over a δ
        // finer than it would have picked, which is the whole
        // difference between a default and a cap.
        self.derived.budget_delta = None;
        self.derived.fit_delta = false;
        self.sync_scene();
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

    /// **Every held node, spoken again from the shown document** after
    /// an operation (`SpokenNode::respoken`'s rule): the form's held face
    /// and the open tool's picks, each saying a node the document no
    /// longer holds by the last label it had.
    fn respeak_held(&mut self) {
        let doc = self.session.doc();
        self.drafts.respeak(doc);
        self.tools.respeak(doc);
    }

    /// **A document replaced this one**: everything held about it is
    /// dropped, here and nowhere else — [`DocumentDerived`] whole, and
    /// the held ids inside the drafts and the open tool, whose values
    /// keep their chrome (typed text, which tool is open) across it.
    fn document_replaced(&mut self) {
        self.derived = DocumentDerived::owed();
        self.drafts.document_replaced();
        self.tools.document_replaced();
    }

    /// Perform one frame's whole batch of operations, keeping the
    /// refusal worth showing.
    ///
    /// **Not one assignment per op.** A frame queues several ops and
    /// several of them can refuse; assigning `status` from each in turn
    /// keeps the LAST, which is how dragging a driven slot came to
    /// display `NoGesture` instead of the ratified affordance (G4) —
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
        // A tool opened, closed or swapped since the last batch — a
        // Cancel in its panel, the previous batch's commit — retires
        // what the line says about the tool that was open. Before the
        // early return: a Cancel is a frame with nothing else to say.
        let open = self.tools.open_kind();
        frame::deliver(
            &mut Vec::new(),
            &mut self.status,
            frame::tool_status(self.line_tool, open),
        );
        self.line_tool = open;
        if ops.is_empty() && notices.is_empty() {
            return;
        }
        let mut refusal: Option<Refusal> = None;
        // The VERDICTS are `frame`'s, computed from the ops and the
        // refusal; this loop only performs and collects (`frame`'s
        // module docs).
        let mut performed: Vec<SessionOp> = Vec::with_capacity(ops.len());
        let mut answered_save_choice = false;
        for op in ops {
            performed.push(op.clone());
            let replaced = op.replaces_the_document();
            let tool_edit = self.tools.commits_open_tool(&op);
            let accepted_op = op.clone();
            let answers = answers_a_save_choice(&op);
            let outcome = self.session.perform(op);
            answered_save_choice |= answers && outcome.refusal.is_none();
            // **Where an outcome's news reaches the user**: what this
            // operation's document transition took out of the display
            // state, and what its committed edits did that nobody
            // asked for by name, onto the frame's notices like every
            // other one (`frame::frame_status` carries the argument).
            //
            // ONE call: `frame::outcome_notices` destructures the
            // outcome, so a new field reds there rather than going
            // unsaid here.
            notices.extend(frame::outcome_notices(&outcome));
            // Read before the match below moves the refusal out: a
            // form that just committed a node it will go on referring
            // to learns its id here and nowhere else.
            let made = outcome.made;
            match outcome.refusal {
                Some(next) => refusal = Refusal::preferred(refusal, next),
                None if replaced => self.document_replaced(),
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
                    self.drafts
                        .creation_landed(self.session.committed_doc(), made);
                }
                // A form whose op committed comes to rest, for the
                // tool's reason: a refusal leaves it holding its draft.
                None => {
                    self.drafts.accepted(&accepted_op);
                    self.drafts
                        .creation_landed(self.session.committed_doc(), made);
                }
            }
            // After the match, so a replaced document's held picks are
            // dropped before anything is spoken from it.
            self.respeak_held();
        }
        let refusal = frame::batch_refusal(refusal, &performed, self.session.committed_doc());
        // A refusal that offers a choice stands it up; a save, fork or
        // replacement that LANDED answers it; any other refusal leaves
        // the standing one where it is.
        if let Some(choice) = refusal.as_ref().and_then(Refusal::save_choice) {
            self.save_choice = Some(choice);
        } else if answered_save_choice {
            self.save_choice = None;
        }
        let verdict = frame::frame_status(&notices, &performed, refusal.as_ref());
        // The refuse-then-offer pair for a parse refusal: hold the
        // refused text in the field it was typed into so acting on the
        // refusal does not cost it, and — for an unknown variable
        // name — prefill the add-variable affordance with the name it
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
            self.drafts.new_variable_name = name.as_str().to_owned();
            self.drafts.new_variable_dimension = None;
            self.drafts.new_variable_offer = Some(name.clone());
            self.drafts.new_variable_open = true;
        }
        self.apply_status(verdict);
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
        // chooser (#2275, 2026-09-10). Nothing is discarded here because nothing is
        // attempted.
        if self.store.unusable().is_some() {
            return;
        }
        let notation = self.session.notation();
        let prefs = Prefs {
            theme: Some(self.theme.name.to_owned()),
            keys: self.keys_pref.clone(),
            last_dir: self.last_dir.clone(),
            length_unit: Some(notation.length.def().symbol().to_owned()),
            angle_unit: Some(notation.angle.def().symbol().to_owned()),
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
        let mut dialog =
            rfd::FileDialog::new().add_filter("document", &[pncad::workspace::DOC_EXTENSION]);
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
                dialog.save_file().map(with_doc_extension)
            }
        }?;
        self.remember_dir(&path);
        Some(path)
    }

    /// This application's door onto [`frame::apply`], for the verdict
    /// the ranking has ALREADY WEIGHED: `perform_batch` hands
    /// [`crate::frame::frame_status`]'s answer here rather than assigning the
    /// field. Its one live caller, and deliberately so — a `Show` that
    /// has been through the ranking must reach the field and must not
    /// be ranked a second time. The types hold both halves:
    /// [`frame::deliver`], which would push it back onto `notices`,
    /// does not take a [`RankedVerdict`], and this does not take a
    /// policy's `frame::StatusUpdate`.
    ///
    /// This is the `&mut self` shorthand for [`crate::frame::apply`],
    /// nothing more. [`crate::pane::viewport`]'s policies reach the
    /// field through [`frame::deliver`], with the `&mut
    /// Option<frame::Message>` it lends them.
    fn apply_status(&mut self, verdict: RankedVerdict) {
        frame::apply(&mut self.status, verdict);
    }

    /// **The advisory-check findings, in a window a reader can keep
    /// open.**
    ///
    /// The badge in the toolbar says how many there are; this says
    /// what they are. Each finding renders through its OWN `Display`
    /// — one composed sentence carrying its subject, its story and
    /// its recourse (`editor_core::finding`) — so the window states
    /// no vocabulary of its own, and beside it sits the one thing
    /// chrome can add: a jump to the body the finding is about, which
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
            .show(ctx, |ui| {
                match self.session.checks().zip(self.session.landed_pair()) {
                    None => {
                        ui.label("nothing has been checked yet");
                    }
                    Some((report, (landed, _))) => {
                        if report.findings.is_empty() && report.pending.is_empty() {
                            ui.label("checks: no findings");
                        }
                        for row in frame::check_rows(report, landed) {
                            ui.horizontal_top(|ui| {
                                if ui
                                    .button(row.button)
                                    .on_hover_text("select what this finding is about")
                                    .clicked()
                                {
                                    ops.push(SessionOp::Select(row.select));
                                }
                                // A sentence, so `widgets::message`.
                                crate::widgets::message(ui, row.sentence);
                            });
                            if let Some(node) = row.add_to_product
                                && ui
                                    .button(crate::pane::properties::ADD_TO_THE_PRODUCT)
                                    .clicked()
                            {
                                ops.push(SessionOp::AddToProduct { node, body: None });
                            }
                            for (label, select) in row.cells {
                                if ui
                                    .button(label)
                                    .on_hover_text("select this cell of the coincidence")
                                    .clicked()
                                {
                                    ops.push(SessionOp::Select(select));
                                }
                            }
                        }
                        if let Some(line) = frame::pending_line(report) {
                            ui.separator();
                            crate::widgets::message_toned(
                                ui,
                                line,
                                &self.theme,
                                frame::Tone::Advisory,
                            );
                        }
                        if !report.skipped.is_empty() {
                            ui.separator();
                            // A sentence too, and one whose length grows
                            // with the number of checks turned off.
                            crate::widgets::message_toned(
                                ui,
                                format!(
                                    "not run (severity Off): {}",
                                    report
                                        .skipped
                                        .iter()
                                        .map(ToString::to_string)
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                ),
                                &self.theme,
                                frame::Tone::Advisory,
                            );
                        }
                    }
                }
            });
        self.checks_shown = open;
    }

    /// The toolbar row: the document's controls, and the two gesture
    /// cancel doors while their gestures are in flight.
    ///
    /// **Wrapped, not one line.** A non-wrapping row is laid out on
    /// one line and clipped, so a window narrower than the row's
    /// natural width does not shrink the right-hand end — it puts it
    /// out of reach, with nothing saying so. A cancel door in flight
    /// sits in this row, and it is the ONE exit from a state in which
    /// every other operation refuses [`Refusal::GestureInFlight`],
    /// which is why it is in the panel that is always drawn. Wrapping
    /// costs a second line only at widths where the alternative was a
    /// control nobody could click.
    ///
    /// **The status line is a row of its own, under the controls, and
    /// that row scrolls.** It is a [`crate::widgets::message`], whose
    /// floor assumes a region that scrolls, and the panel does not; so
    /// the row is a horizontal scroll area the width of the panel, and
    /// a window narrower than the floor scrolls the line rather than
    /// drawing it past the panel. The row also fixes where the line
    /// begins: at the panel's left edge, at every width, rather than
    /// wherever the wrapping row's cursor fell.
    ///
    /// **The document's name is truncated at the panel's edge**, with
    /// the whole name on hover: the user chose it, so nothing here
    /// bounds its width, and it is a name rather than a sentence.
    ///
    /// [`Refusal::GestureInFlight`]: crate::session::Refusal::GestureInFlight
    fn toolbar_ui(&mut self, ui: &mut egui::Ui, ops: &mut Vec<SessionOp>, chosen: &mut Theme) {
        // Read before the controls: a control laid out past the
        // panel widens this `Ui`, and the status row (its separator
        // included) must not follow.
        let panel_width = ui.available_width();
        ui.horizontal_wrapped(|ui| {
            // What is OPEN, not what the program is called: the
            // window title already carries the application's name,
            // and a toolbar that repeats it tells a user nothing
            // they cannot see in their own title bar.
            ui.add(egui::Label::new(document_name(self.session.path())).truncate());
            ui.separator();
            // The New… control: one name field. The field is a draft; the
            // op is emitted only by its commit, gated on
            // `Refusal::new_document_name`, the door's own blank check,
            // so the button cannot offer a click the door refuses. Out
            // of it the button shows that refusal's own words.
            match self.drafts.new_doc_name.as_mut() {
                None => {
                    if crate::widgets::door(ui, "New…").clicked() {
                        self.drafts.new_doc_name = Some(String::new());
                    }
                }
                Some(name) => {
                    ui.add(
                        egui::TextEdit::singleline(name)
                            .hint_text("document name")
                            .desired_width(120.0),
                    );
                    let named = Refusal::new_document_name(name).map(str::to_owned);
                    let blocked = named.as_ref().err().map(Refusal::to_string);
                    let closing = crate::widgets::commit_row(
                        ui,
                        NEW_DOCUMENT_COMMIT,
                        blocked.as_deref(),
                        |_| {},
                    );
                    if closing.commit
                        && let Ok(name) = named
                    {
                        ops.push(SessionOp::NewDocument { name });
                    }
                    if closing.commit || closing.cancel {
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
            let no_chooser = self.chooser.unusable();
            let open = match no_chooser {
                Some(reason) => ui
                    .add_enabled(false, egui::Button::new("Open…"))
                    .on_disabled_hover_text(reason),
                None => ui.button("Open…"),
            };
            if open.clicked() {
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
            // Save writes at the session's own path and needs no
            // dialog; with no path it is Save As….
            let save = match (self.session.path(), no_chooser) {
                (None, Some(reason)) => ui
                    .add_enabled(false, egui::Button::new("Save"))
                    .on_disabled_hover_text(reason),
                _ => ui.button("Save"),
            };
            if save.clicked() {
                let path = self.session.path().map(std::path::Path::to_path_buf);
                // The dialog is unreachable on wasm, for the reason the
                // Open… arm above states.
                #[cfg(not(target_family = "wasm"))]
                let path = path.or_else(|| self.file_dialog(FileDialog::SaveAs));
                if let Some(path) = path {
                    ops.push(SessionOp::Save(path));
                }
            }
            let save_as = match no_chooser {
                Some(reason) => ui
                    .add_enabled(false, egui::Button::new("Save As…"))
                    .on_disabled_hover_text(reason),
                None => ui.button("Save As…"),
            };
            if save_as.clicked() {
                // Unreachable on wasm, for the reason the Open…
                // arm above states.
                #[cfg(not(target_family = "wasm"))]
                if let Some(path) = self.file_dialog(FileDialog::SaveAs) {
                    ops.push(SessionOp::Save(path));
                }
            }
            if let Some(choice) = self.save_choice.clone() {
                ui.separator();
                if ui.button(choice.over_label()).clicked() {
                    ops.push(choice.over());
                }
                if ui.button(choice.fork_label()).clicked() {
                    ops.push(choice.fork());
                }
                if ui.button("Dismiss").clicked() {
                    self.save_choice = None;
                }
            }
            ui.separator();
            // **The history controls**, drawn at rest as well, and
            // disabled rather than absent when they can do nothing: the
            // refusal the step would be answered with decides both
            // whether the button is live and what it says while it is
            // not (`Refusal::nothing_to_step`). These
            // two buttons are the only hand that pushes
            // `SessionOp::Undo` or `SessionOp::Redo`, so a sentence
            // they do not show is one no reader ever meets — and the
            // refusal names ITS OWN direction, because a redo waiting
            // on the branch the cursor just left makes a sentence
            // about both false of the half that is live.
            for (label, direction, op) in [
                ("Undo", Step::Undo, SessionOp::Undo),
                ("Redo", Step::Redo, SessionOp::Redo),
            ] {
                let blocked = Refusal::nothing_to_step(self.session.history(), direction);
                if refusable_button(ui, label, blocked.as_ref()) {
                    ops.push(op);
                }
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
            // Drawn exactly while the door's own gesture is in
            // flight — the door's `blocked`, the refusal its operation
            // gives with no gesture, is the one test — so the toolbar
            // at rest carries no cancel with nothing to cancel.
            let mut doors = self
                .session
                .cancel_doors()
                .into_iter()
                .filter(|door| door.blocked.is_none())
                .peekable();
            if doors.peek().is_some() {
                for door in doors {
                    if ui.button(door.label).clicked() {
                        ops.push(door.op);
                    }
                }
                ui.separator();
            }
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
            match self.progress() {
                Some(frame::Progress::Evaluating) => {
                    ui.separator();
                    ui.spinner();
                    ui.label("evaluating…");
                    if ui.button(crate::widgets::CANCEL).clicked() {
                        ops.push(SessionOp::CancelEvaluation);
                    }
                    // A background result is not a user event, so
                    // nothing else would wake the frame loop to
                    // collect it.
                    ui.ctx().request_repaint();
                }
                Some(frame::Progress::Canceled { index }) => {
                    let indexing = index == IndexSeam::Building;
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
                    // Two clauses in a wrapping row, so a sentence:
                    // `widgets::message`.
                    crate::widgets::message(ui, CANCELED_LINE);
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
            // The door's answer is a background result too; it changes
            // the Checks window and its badge, not the picture, so a
            // slower wake than a spinner's is enough.
            if self.session.proving() {
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(250));
            }
            // **The reads, one draw each.** Each is a function of
            // the typed value it reads — including its silence,
            // which is what lets a row assert the `None`
            // — and each states its own tone and affordance, so
            // nothing about how a badge looks is decided here
            // (`frame::Badge`).
            //
            // The at-rest badge, when the session took one
            // (`DocSession::at_rest`): the verification verdict
            // living past the commit.
            if let Some(badge) = frame::at_rest_badge(self.session.at_rest()) {
                draw_badge(ui, &self.theme, &badge);
            }
            // The advisory checks. It REPORTS: the scene below is
            // drawn either way, because a product whose copies
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
            // Which faults reach it is `frame::badge_site`'s.
            if let Some((landed, _)) = self.session.landed_pair()
                && let Some(badge) = frame::product_badge(self.session.product_fault(), landed)
            {
                draw_badge(ui, &self.theme, &badge);
            }
            // The product's recourse, as the action it names: the one
            // door, on each body drawn outside the product.
            for (label, op) in
                frame::product_recourse(self.session.product_fault(), self.session.committed_doc())
            {
                if ui.button(label).clicked() {
                    ops.push(op);
                }
            }
            // The display budget's: shown while the δ on screen is
            // the one the budget CHOSE when the document opened,
            // and gone the moment the user picks their own.
            if let Some(badge) = frame::delta_badge(
                self.derived.budget_delta.as_ref(),
                self.session.notation().length,
            ) {
                draw_badge(ui, &self.theme, &badge);
            }
            // **The three display seams that hold a refusal.**
            // Each is a read of held state — the scene fault kept
            // until a rebuild lands, the pick cache's own held
            // refusal or the copies its index could not draw, the
            // projection the viewport could not form — so each stands
            // for as long as what it reads holds, rather than until the
            // next acting batch sweeps a line. What a pick aimed at the missing index gets is
            // still the line's, because that is an outcome
            // (`frame::unindexed_refusal`).
            for badge in [
                frame::scene_badge(self.scene_fault.as_ref()),
                frame::index_badge(
                    self.picks.error(),
                    self.picks.index(),
                    self.session.landed_pair(),
                ),
                frame::projection_badge(self.projection_fault.as_ref()),
            ]
            .into_iter()
            .flatten()
            {
                draw_badge(ui, &self.theme, &badge);
            }
            // **The committed profiles the last drawn frame drew
            // nothing of.** Not one of the three above: those hold a
            // refusal until the seam they name succeeds, and this is a
            // count the frame re-takes. A profile left out of the
            // picture is otherwise a document without it.
            if let Some(badge) = frame::profiles_badge(self.viewport_reads.profiles_undrawn) {
                draw_badge(ui, &self.theme, &badge);
            }
            // And the held edges the mark could not tell held or not.
            if let Some(badge) = frame::held_edges_badge(
                self.viewport_reads.held_edges_refused.as_ref(),
                self.session.landed_pair().map(|(doc, _)| doc),
            ) {
                draw_badge(ui, &self.theme, &badge);
            }
            ui.separator();
            // The palette picker. Every registered theme, by the
            // name `crate::theme` gives it — the registry IS the
            // menu, so a theme cannot be shipped and left
            // unreachable.
            //
            // A combo lays itself out at the cursor whether the row has
            // room or not (`egui`'s `button_frame` takes the rect before
            // the wrap), so the wrapping row is asked first, against the
            // width the picker took when it was last drawn: a picker that
            // would run past the edge starts the next line.
            let picker_id = ui.id().with("viewer_theme_width");
            let last_width: Option<f32> = ui.ctx().data(|data| data.get_temp(picker_id));
            if last_width.is_some_and(|width| ui.available_size_before_wrap().x < width) {
                ui.end_row();
            }
            let picker = egui::ComboBox::from_id_salt("viewer_theme")
                .selected_text(chosen.name)
                .show_ui(ui, |ui| {
                    for theme in Theme::ALL {
                        ui.selectable_value(&mut *chosen, *theme, theme.name);
                    }
                });
            let width = picker.response.rect.width() + ui.spacing().item_spacing.x;
            ui.ctx().data_mut(|data| data.insert_temp(picker_id, width));
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
        });
        if let Some(status) = &self.status {
            ui.scope(|ui| {
                ui.set_max_width(panel_width);
                ui.separator();
                egui::ScrollArea::horizontal()
                    .id_salt("viewer_status_line")
                    .show(ui, |ui| crate::widgets::message(ui, status.text()));
            });
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
            self.toolbar_ui(ui, &mut ops, &mut chosen);
        });

        if chosen != self.theme {
            self.theme = chosen;
            apply_polarity(ui.ctx(), chosen.polarity);
            self.remember_prefs();
        }

        let display = self.session.display_view();
        let toggles = self.session.toggles();
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
            self.session.doc(),
            self.session.tol(),
            self.session.selection().node(),
        );
        let held = self.drafts.door_loops();
        // Both doors draw on the profile's own plane
        // ([`sketch::profile_plane`]).
        let (tol, delta) = (self.session.tol(), self.delta.get());
        let mut profile_previews = held.as_ref().zip(self.profile_drawn).map(|(held, drawn)| {
            held.as_ref()
                .filter(|_| drawn)
                .map(|held| sketch::preview(sketch::profile_plane(), &held.loops, tol, delta))
        });
        // A traced path runs onto the trace at evaluation, which no
        // replay here can draw: the create door shows the trace instead.
        let landed = self.session.evaluation_arc().cloned();
        if let Some(latches) = self
            .drafts
            .trace_wanted(self.profile_drawn.create, landed.as_ref())
        {
            let read = self.session.sketch_trace(&latches.0, &latches.1);
            self.drafts.show_trace(latches, landed.as_ref(), read);
        }
        if self.profile_drawn.create && self.drafts.traces() {
            profile_previews.create = self
                .drafts
                .trace_shown
                .as_ref()
                .and_then(|shown| shown.read.as_ref().ok())
                .map(|marks| Ok(sketch::trace_preview(marks)));
        }
        // The committed node the edit door is previewing in its place,
        // which the committed-profile pass leaves out.
        let profile_edited = self.drafts.edited_in_place(profile_previews.edit.as_ref());
        let mut profile_drawn = ProfileDoors::<bool>::default();
        // **Zeroed here, every frame: after the toolbar has read last
        // frame's value and before the panes draw.** A frame the
        // viewport does not draw at all is a frame it found nothing in,
        // and this is where that is said rather than left to whatever
        // the last drawn frame found.
        self.viewport_reads = ViewportReads::default();
        let mut delta_request: Option<DisplayTolerance> = None;
        let mut features_content_height: Option<f32> = None;
        let mut split_dragged = self.split_dragged;
        // The working notation the panes read and the forms' pickers
        // write, taken from the session and given back after the
        // layout like the palette: a pick is a person's preference,
        // never a `SessionOp`, because it changes nothing a document
        // says.
        let mut notation = self.session.notation();
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
        // The revolve tool's held profile as the viewport will draw its
        // axis over it, for the repaint request after the picks are fed.
        let revolve_held = self.tools.revolve().and_then(|tool| tool.profile());
        let split_held = self.tools.split().map(|tool| tool.seats().clone());
        egui::CentralPanel::no_frame()
            .frame(egui::Frame::NONE.fill(ui.visuals().panel_fill))
            .show(ui, |ui| {
                let mut behavior = ViewerBehavior {
                    session: &self.session,
                    delta: self.delta,
                    budget_delta: self.derived.budget_delta,
                    scene: &self.scene,
                    index: self.picks.index(),
                    scene_key: self.scene_key.clone(),
                    indexing: self.picks.index_seam(self.fit.as_ref()),
                    revision: self.revision,
                    camera: &mut self.camera,
                    input: self.input,
                    theme: self.theme,
                    notation: &mut notation,
                    drafts: &mut self.drafts,
                    display: &display,
                    toggles: &toggles,
                    tools: &mut self.tools,
                    part_chooser: &mut self.part_chooser,
                    profile_previews: &profile_previews,
                    profile_drawn: &mut profile_drawn,
                    profile_was_drawn: self.profile_drawn,
                    profile_edited,
                    pending_fit: &mut self.pending_fit,
                    projection_fault: &mut self.projection_fault,
                    viewport_reads: &mut self.viewport_reads,
                    notices: &mut self.notices,
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
        self.checks_window(ui.ctx(), &mut ops);
        if notation != self.session.notation() {
            self.session.set_notation(notation);
            self.remember_prefs();
        }
        self.profile_drawn = profile_drawn;
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
        // GUI plan's two-sequential-picks ruling — the same single-select value,
        // copied into tool state). Which vocabulary each tool reads is
        // `Tools::feed`'s to know, and a pick a tool DECLINED comes
        // back as a notice through the same door as a survival drop.
        // A declined pick answers an act the user aimed at the
        // document, like every other rank-2 notice this frame.
        let declined = self.tools.feed(
            self.session.doc(),
            self.picks.index_or_why(self.fit.as_ref()),
            &ops,
        );
        // A revolve profile picked or cleared after the viewport drew
        // this frame's axis (`pane::viewport::push_revolve_axis`) asks
        // for the next.
        if self.tools.revolve().and_then(|tool| tool.profile()) != revolve_held {
            ui.ctx().request_repaint();
        }
        // The split plane likewise (`splitplane::push_split_plane`): a
        // target picked or cleared after it was drawn.
        if self.tools.split().map(|tool| tool.seats().clone()) != split_held {
            ui.ctx().request_repaint();
        }
        let landed = self.session.landed_pair();
        self.notices.extend(
            declined
                .iter()
                .map(|notice| frame::tool_notice(notice, landed)),
        );

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

/// **What the viewport found while it drew one frame**: the per-frame,
/// unlatched reads the toolbar badges, one [`crate::frame`] badge door
/// per member.
///
/// `Default` is the frame the viewport did not draw. The frame entry
/// point (`<ViewerApp as eframe::App>::ui`) resets the whole value to
/// it before the panes draw, so a member says what the LAST frame found
/// and never what an earlier one did: a viewport dragged shut or tabbed
/// away reports nothing. That reset names no member, so a member added
/// here is zeroed by the line that zeroes the rest.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct ViewportReads {
    /// How many committed profiles the viewport drew nothing of
    /// (`crate::sketch::CommittedProfiles::undrawn`), for
    /// [`frame::profiles_badge`].
    pub(crate) profiles_undrawn: usize,
    /// What the index could not name on the held edge set's body
    /// (`marks::EdgeOverlay::held_refused`), for
    /// [`frame::held_edges_badge`].
    pub(crate) held_edges_refused: Option<frame::Spelled<EdgeNamesRefused>>,
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
    /// Whether an index for the picture this frame WANTS is on its way
    /// — the other half of what `index: None` means, and the half that
    /// decides which sentence a refused pick gets
    /// ([`crate::pickcache::NotIndexed`]). The same value the toolbar's
    /// progress state reads, from the same door, so a click and the
    /// spinner cannot describe one wait two ways.
    pub(crate) indexing: IndexSeam,
    pub(crate) revision: u64,
    pub(crate) camera: &'a mut Camera,
    pub(crate) input: InputMap,
    /// The palette this frame draws with; `Copy`, because a theme is
    /// a small value and the frame must not be able to change it.
    pub(crate) theme: Theme,
    /// The working notation ([`crate::props::Notation`]) every value
    /// nobody wrote reads in this frame; the creation forms' unit
    /// pickers write it, and the app hands a changed one to the session
    /// and the preferences after the layout.
    pub(crate) notation: &'a mut crate::props::Notation,
    pub(crate) drafts: &'a mut Drafts,
    /// The display snapshot this frame draws and picks under.
    pub(crate) display: &'a DisplayView,
    /// Every show/hide toggle, read once over the committed document:
    /// what the View pane lists and the tree's and the Properties
    /// pane's checkboxes read.
    pub(crate) toggles: &'a crate::display::Toggles,
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
    /// What [`Self::profile_drawn`] said on the last frame: which doors
    /// are on screen, the one read every preview and mark of a door
    /// follows.
    pub(crate) profile_was_drawn: ProfileDoors<bool>,
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
    /// What the viewport finds while it draws this frame, for the
    /// toolbar to badge next frame ([`ViewerApp::viewport_reads`]).
    /// Defaulted by the frame entry point before the panes draw, so no
    /// writer has to clear it.
    pub(crate) viewport_reads: &'a mut ViewportReads,
    /// **What this frame's panes have to SAY**, joined and ranked by
    /// [`frame::frame_status`] with everything else the frame
    /// produced. A pane that assigned `status` instead had no way to
    /// say "I have nothing to add", and its sentence was erased by the
    /// batch this frame accepted before it was ever painted —
    /// `perform_batch` runs after the panes have drawn.
    pub(crate) notices: &'a mut Vec<frame::Message>,
    /// The line itself, for the one thing a notice cannot do: RETIRE a
    /// sentence. [`crate::frame::cursor_status`] and a clean camera fold expire
    /// what they last said and add nothing, so neither retirement is
    /// ranked: [`frame::deliver`], the one door a policy's verdict
    /// fits, writes each retirement here and sends a refused fold's
    /// news to `notices` above ([`crate::pane::viewport`]: `land`, and
    /// the id pass).
    pub(crate) status: &'a mut Option<frame::Message>,
    pub(crate) id_answer: &'a IdChannel,
    pub(crate) id_log: &'a mut IdQueryLog,
    pub(crate) ops: &'a mut Vec<SessionOp>,
    /// A δ the View pane's field committed this frame. The pane holds
    /// a borrow of the app, not the app, so it hands the δ back for
    /// [`ViewerApp::set_delta`] to apply.
    pub(crate) delta_request: &'a mut Option<DisplayTolerance>,
    /// What the Features pane's content laid out to this frame, once
    /// it has drawn.
    pub(crate) features_content_height: &'a mut Option<f32>,
    /// Set when the user resized a tile themselves.
    pub(crate) split_dragged: &'a mut bool,
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

/// **Whether `op`, landing, answers a standing [`SaveChoice`]**: a
/// save or a fork, and every door that puts another document under the
/// session.
fn answers_a_save_choice(op: &SessionOp) -> bool {
    op.replaces_the_document() || matches!(op, SessionOp::Save(_) | SessionOp::SaveAsNewDocument(_))
}

/// **A chosen save path, as the store will see it**: a `.pncad` file.
/// A name typed without the extension gains it, since the Linux
/// dialogs return what was typed (`rfd`'s GTK and portal backends
/// append nothing).
#[cfg(not(target_family = "wasm"))]
fn with_doc_extension(path: std::path::PathBuf) -> std::path::PathBuf {
    use pncad::workspace::DOC_EXTENSION;
    if path.extension().is_some_and(|ext| ext == DOC_EXTENSION) {
        return path;
    }
    let mut named = path.into_os_string();
    named.push(".");
    named.push(DOC_EXTENSION);
    named.into()
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
    #![allow(clippy::panic)]

    use super::{CANCELED_LINE, FEATURES_SHARE_CAP, Polarity, Theme, ViewerApp, features_fraction};
    use crate::session::{SaveChoice, SessionOp};
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
        /// The panel's own rectangle — the region a status line has to
        /// stay inside, and whose left edge is where egui would put a
        /// wrapped label's second line.
        panel: egui::Rect,
        /// Where the sentence [`toolbar_drawn`] looked for landed, one
        /// rect per line, empty when it was given none.
        status: Vec<egui::Rect>,
        /// Every text run the second frame painted, in paint order.
        painted: Vec<String>,
        /// The clip rect that sentence was painted under: the part of
        /// a line outside it is not on screen.
        status_clip: egui::Rect,
        /// The right-hand end of the widest HORIZONTAL rule that
        /// starts inside the panel: the status row's separator, the
        /// only one the toolbar draws (the controls' separators are
        /// vertical, and the panel's own border starts at the
        /// window's edge). `-inf` for none.
        rule_right: f32,
        /// [`crate::widgets::message_floor`] in the panel.
        floor: f32,
    }

    /// A status line longer than a narrow window's toolbar row, in the
    /// shape the chrome actually shows: a refusal's own words.
    const STATUS: &str =
        "the chain does not close yet — its last step has to target the start of the loop it began";

    /// Lines are placed at whole pixels, so two readings of one edge
    /// can differ by less than one.
    const SLACK: f32 = 1.0;

    /// Lay the real toolbar out in a headless context whose window is
    /// `width` points wide.
    ///
    /// Two frames: the first is the one egui sizes from defaults, the
    /// second is the one a user looks at.
    fn toolbar_row(width: f32) -> Row {
        toolbar_with(width, None)
    }

    /// [`toolbar_row`], with `status` on the line if there is one —
    /// the state Ev's second symptom is about, which the statusless
    /// frame never reaches.
    fn toolbar_with(width: f32, status: Option<&str>) -> Row {
        toolbar_drawn(
            width,
            |app| {
                app.status = status.map(|text| {
                    crate::frame::Message::new(
                        crate::frame::Subject::Document,
                        text,
                        crate::frame::Retold::Again,
                    )
                });
            },
            status,
        )
    }

    /// The toolbar in a window `width` points wide, over an app
    /// `prepare` put in the state a row is about, with the lines
    /// `sentence` landed in read off the second frame.
    fn toolbar_drawn(
        width: f32,
        prepare: impl FnOnce(&mut ViewerApp),
        sentence: Option<&str>,
    ) -> Row {
        toolbar_driven(width, prepare, sentence, |_| Vec::new())
    }

    /// [`toolbar_drawn`], then one frame per batch of events `input`
    /// hands back — it is shown the row the frame before measured,
    /// and an empty batch ends the drive — with `sentence` read off
    /// the last frame.
    fn toolbar_driven(
        width: f32,
        prepare: impl FnOnce(&mut ViewerApp),
        sentence: Option<&str>,
        mut input: impl FnMut(&Row) -> Vec<egui::Event>,
    ) -> Row {
        let ctx = egui::Context::default();
        let mut app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
            .expect("startup that needs no graphics device");
        prepare(&mut app);
        let mut row = Row {
            occupied: f32::NAN,
            available: f32::NAN,
            panel: egui::Rect::NOTHING,
            status: Vec::new(),
            painted: Vec::new(),
            status_clip: egui::Rect::NOTHING,
            rule_right: f32::NEG_INFINITY,
            floor: f32::NAN,
        };
        let mut frame = 0;
        loop {
            let events = if frame < 2 { Vec::new() } else { input(&row) };
            if frame >= 2 && events.is_empty() {
                break;
            }
            frame += 1;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 600.0),
                )),
                events,
                ..Default::default()
            };
            let app = &mut app;
            let row = &mut row;
            let mut output = ctx.run_ui(input, |ui| {
                egui::Panel::top("viewer_toolbar").show(ui, |ui| {
                    let mut ops: Vec<SessionOp> = Vec::new();
                    let mut chosen = Theme::ALL[0];
                    row.available = ui.available_width();
                    row.panel = ui.max_rect();
                    row.floor = crate::widgets::message_floor(ui);
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
            let painted = crate::pane::headless::landed_in(&output.shapes);
            row.painted = painted.iter().map(|landed| landed.text.clone()).collect();
            let landed =
                sentence.and_then(|text| painted.into_iter().find(|landed| landed.text == text));
            let panel = row.panel;
            row.rule_right = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::LineSegment { points: [a, b], .. }
                        if a.y == b.y && a.x.min(b.x) >= panel.left() - SLACK =>
                    {
                        Some(a.x.max(b.x))
                    }
                    _ => None,
                })
                .fold(f32::NEG_INFINITY, f32::max);
            (row.status, row.status_clip) = landed.map_or_else(
                || (Vec::new(), egui::Rect::NOTHING),
                |landed| (landed.rows, landed.clip),
            );
            // Nothing here paints, so the frame's texture delta is
            // dropped rather than uploaded, and epaint refuses a drop
            // it did not see taken.
            output.textures_delta.clear();
        }
        row
    }

    /// Where the toolbar's canceled line landed in a window `width`
    /// points wide, over a session a cancel left showing an older
    /// result.
    fn canceled_line(width: f32) -> Row {
        toolbar_drawn(
            width,
            |app| {
                let tol = pncad::tolerance::witness();
                let (document, _) =
                    crate::scene::plate_with_hole(tol).expect("the startup document");
                let mut session = crate::session::DocSession::inline(document, tol);
                session.pump();
                session.perform(SessionOp::Reevaluate);
                session.perform(SessionOp::CancelEvaluation);
                session.pump();
                assert_eq!(
                    session.outstanding(),
                    crate::session::Outstanding::Canceled,
                    "the fixture is the state the canceled line is drawn in"
                );
                app.session = session;
            },
            Some(CANCELED_LINE),
        )
    }

    /// **What the toolbar draws for each progress state**, planted
    /// through the reads it takes them from: the session's
    /// [`crate::session::Outstanding`] and the index seam, whose
    /// building state is planted as a fit in flight — the one the
    /// cache's own record cannot see.
    ///
    /// The spinner is not text and is not read here; what is asserted
    /// is the words and the recourse beside them, in paint order.
    #[test]
    fn the_toolbar_draws_each_progress_state_once() {
        use crate::evalseam::{FitService, InlineFitter};
        use crate::pickcache::IndexSeam;
        use crate::session::{DocSession, Outstanding};

        const WORDS: [&str; 5] = [
            "evaluating…",
            "Cancel",
            CANCELED_LINE,
            "Re-evaluate",
            "indexing…",
        ];
        let tol = pncad::tolerance::witness();
        let session = |outstanding: Outstanding| {
            let (document, _) = crate::scene::plate_with_hole(tol).expect("the startup document");
            let mut session = DocSession::inline(document, tol);
            session.pump();
            match outstanding {
                Outstanding::Current => {}
                Outstanding::Evaluating => {
                    session.perform(SessionOp::Reevaluate);
                }
                Outstanding::Canceled => {
                    session.perform(SessionOp::Reevaluate);
                    session.perform(SessionOp::CancelEvaluation);
                    session.pump();
                }
            }
            assert_eq!(
                session.outstanding(),
                outstanding,
                "the fixture plants {outstanding:?}"
            );
            session
        };
        let drawn = |outstanding: Outstanding, index: IndexSeam| {
            let row = toolbar_drawn(
                UNBOUNDED,
                |app| {
                    app.session = session(outstanding);
                    let mut fit = InlineFitter::new();
                    if index == IndexSeam::Building {
                        fit.submit(
                            app.session
                                .fit_request(app.delta)
                                .expect("a landed plate has a body to fit"),
                        );
                    }
                    app.fit = Box::new(fit);
                    assert_eq!(
                        app.picks.index_seam(app.fit.as_ref()),
                        index,
                        "the fixture plants {index:?}"
                    );
                },
                None,
            );
            row.painted
                .into_iter()
                .filter(|text| WORDS.contains(&text.as_str()))
                .collect::<Vec<_>>()
        };
        for (outstanding, index, expected) in [
            (Outstanding::Current, IndexSeam::Idle, vec![]),
            (Outstanding::Current, IndexSeam::Building, vec!["indexing…"]),
            (
                Outstanding::Evaluating,
                IndexSeam::Idle,
                vec!["evaluating…", "Cancel"],
            ),
            (
                Outstanding::Evaluating,
                IndexSeam::Building,
                vec!["evaluating…", "Cancel"],
            ),
            (
                Outstanding::Canceled,
                IndexSeam::Idle,
                vec![CANCELED_LINE, "Re-evaluate"],
            ),
            (
                Outstanding::Canceled,
                IndexSeam::Building,
                vec![CANCELED_LINE, "Re-evaluate", "indexing…"],
            ),
        ] {
            assert_eq!(
                drawn(outstanding, index),
                expected,
                "the toolbar over {outstanding:?} with the index seam {index:?}"
            );
        }
    }

    /// **The canceled line wraps whole**: at the row's own left edge,
    /// and never split between the end of one line of the toolbar and
    /// the start of the next.
    ///
    /// It sits in the toolbar's wrapping row, where
    /// `egui::Label::layout_in_ui` starts a label beside the widget
    /// before it and puts its second line at the panel's left edge.
    /// Where the line falls on the row moves with the window, so the
    /// row sweeps the width across the range a phone and a half-tiled
    /// desktop window give, rather than guessing one width where the
    /// cursor happens to sit near the end of a line.
    #[test]
    fn the_toolbars_canceled_line_begins_every_line_in_the_same_place() {
        for width in (0..=16).map(|step| NARROW / 2.0 + 25.0 * step as f32) {
            let row = canceled_line(width);
            assert!(
                !row.status.is_empty(),
                "the canceled line was painted at a {width}-point window"
            );
            let first = row.status[0].left();
            let drift = row
                .status
                .iter()
                .map(|line| (line.left() - first).abs())
                .fold(f32::NEG_INFINITY, f32::max);
            assert!(
                drift <= SLACK,
                "at a {width}-point window every line of the canceled line \
                 begins under the first ({drift} points of drift across {:?})",
                row.status
            );
        }
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

    /// Every text run a headless frame painted, with the rect it
    /// occupies.
    ///
    /// The toolbar's own labels, and — when the pointer is resting on
    /// a control that cannot be used — the words
    /// `on_disabled_hover_text` puts in front of the reader. Painting
    /// is the only place those words exist: `add_enabled` and
    /// `on_disabled_hover_text` hand back a `Response` and keep no
    /// value, so a row that does not read the frame can assert the
    /// refusal a control was BUILT from and never which control got
    /// it.
    fn painted_text(shapes: &[egui::epaint::ClippedShape]) -> Vec<(String, egui::Rect)> {
        fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
            match shape {
                egui::Shape::Text(text) => out.push((
                    text.galley.text().to_owned(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                )),
                egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
                _ => {}
            }
        }
        let mut out = Vec::new();
        for clipped in shapes {
            walk(&clipped.shape, &mut out);
        }
        out
    }

    /// The real toolbar in a headless context, with a pointer that can
    /// be rested on one of its controls.
    struct Toolbar {
        ctx: egui::Context,
        app: ViewerApp,
        /// Frames are numbered in seconds because a tooltip is a
        /// function of how long the pointer has been still.
        time: f64,
    }

    impl Toolbar {
        fn open() -> Self {
            let ctx = egui::Context::default();
            // A tooltip this row waited half a second for would be a
            // row about `tooltip_delay`. Nothing else about the hover
            // changes.
            ctx.all_styles_mut(|style| style.interaction.tooltip_delay = 0.0);
            let app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
                .expect("startup that needs no graphics device");
            Self {
                ctx,
                app,
                time: 0.0,
            }
        }

        /// One frame of the real [`ViewerApp::toolbar_ui`], with the
        /// pointer where the caller puts it. Answers what was painted.
        fn frame(&mut self, pointer: Option<egui::Pos2>) -> Vec<(String, egui::Rect)> {
            painted_text(&self.shapes(pointer))
        }

        /// [`Self::frame`]'s drive, answering the shapes themselves.
        fn shapes(&mut self, pointer: Option<egui::Pos2>) -> Vec<egui::epaint::ClippedShape> {
            self.time += 1.0;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1600.0, 600.0),
                )),
                time: Some(self.time),
                events: pointer.map(egui::Event::PointerMoved).into_iter().collect(),
                ..Default::default()
            };
            let Self { ctx, app, .. } = self;
            let mut output = ctx.run_ui(input, |ui| {
                egui::Panel::top("viewer_toolbar").show(ui, |ui| {
                    let mut ops: Vec<SessionOp> = Vec::new();
                    let mut chosen = Theme::ALL[0];
                    app.toolbar_ui(ui, &mut ops, &mut chosen);
                });
            });
            // Nothing here paints, so the frame's texture delta is
            // dropped rather than uploaded.
            output.textures_delta.clear();
            output.shapes
        }

        /// **Whether the control labelled `label` is drawn live**:
        /// its label painted in the same ink as `New…`'s, which is
        /// drawn unconditionally live, rather than in the faded ink
        /// egui paints a disabled control's label with. Read with
        /// nothing hovered, so both labels are in their resting ink.
        ///
        /// A hover alone cannot tell a live control from a disabled
        /// one with no reason attached: neither shows anything.
        fn drawn_live(&mut self, label: &str) -> bool {
            self.shapes(Some(Self::ELSEWHERE));
            let landed = crate::pane::headless::landed_in(&self.shapes(Some(Self::ELSEWHERE)));
            let ink = |wanted: &str| {
                landed
                    .iter()
                    .find(|landed| landed.text == wanted)
                    .unwrap_or_else(|| panic!("the toolbar draws a control labelled {wanted}"))
                    .ink
            };
            ink(label) == ink("New…")
        }

        /// **What the toolbar puts in front of a reader who rests the
        /// pointer on the control labelled `label`** — `None` when the
        /// control is live and has nothing to explain.
        ///
        /// Read as the difference between a quiet frame and a hovered
        /// one, so it names no wording of its own: whatever the hover
        /// ADDED to the picture is what the reader gained by hovering.
        fn reason_shown_on(&mut self, label: &str) -> Option<String> {
            // The pointer is parked off the toolbar first, so that a
            // tooltip left over from the previous control is not in
            // the picture this one is measured against. egui keeps the
            // last position it was given, so "no event" is not "no
            // pointer".
            self.frame(Some(Self::ELSEWHERE));
            let quiet = self.frame(Some(Self::ELSEWHERE));
            let rect = quiet
                .iter()
                .find(|(text, _)| text == label)
                .unwrap_or_else(|| {
                    panic!("the toolbar draws a control labelled {label}: {quiet:?}")
                })
                .1;
            // Two frames with the pointer on it: egui decides hover
            // against the rect the previous frame left behind.
            self.frame(Some(rect.center()));
            let hovered = self.frame(Some(rect.center()));
            let before: Vec<&str> = quiet.iter().map(|(text, _)| text.as_str()).collect();
            let gained: Vec<String> = hovered
                .iter()
                .map(|(text, _)| text.clone())
                .filter(|text| !before.contains(&text.as_str()))
                .collect();
            assert!(
                gained.len() <= 1,
                "hovering {label} added more than one text run, so this row cannot say which is \
                 the reason: gained {gained:?}, quiet {quiet:?}, hovered {hovered:?}"
            );
            gained.into_iter().next()
        }

        /// A point inside the window and off the toolbar, for a frame
        /// in which nothing is hovered.
        const ELSEWHERE: egui::Pos2 = egui::Pos2::new(1500.0, 550.0);
    }

    /// **A cancel door is on the toolbar exactly while its gesture is
    /// in flight**: neither at rest, and the drag's door once a slot
    /// drag has begun — its in-flight state read off the door
    /// (`CancelDoor::blocked`), so the free-move door stays off while
    /// only the drag is held.
    #[test]
    fn a_cancel_door_is_on_the_toolbar_only_while_its_gesture_is_in_flight() {
        const DRAG: &str = "Cancel drag";
        const FREE_MOVE: &str = "Cancel free-move";
        let shown = |painted: &[(String, egui::Rect)], label: &str| {
            painted.iter().any(|(text, _)| text == label)
        };
        let mut toolbar = Toolbar::open();
        let rest = toolbar.frame(Some(Toolbar::ELSEWHERE));
        assert!(
            !shown(&rest, DRAG) && !shown(&rest, FREE_MOVE),
            "no gesture in flight, no cancel on the toolbar: {rest:?}"
        );
        // The startup plate's extrude (`scene::plate_with_hole`).
        let extrude = toolbar.app.session.doc().ids()[1];
        let begun = toolbar.app.session.perform(SessionOp::BeginGesture {
            node: extrude,
            slot: pncad::document::SlotId::Distance,
        });
        assert!(begun.refusal.is_none(), "the drag begins: {begun:?}");
        let held = toolbar.frame(Some(Toolbar::ELSEWHERE));
        assert!(
            shown(&held, DRAG) && !shown(&held, FREE_MOVE),
            "a drag in flight puts its own door on the toolbar and no other: {held:?}"
        );
    }

    /// **A disabled toolbar control says what its own operation
    /// refuses** — and says it about ITSELF.
    ///
    /// Three controls are held to it: Undo, Redo, and the commit on the
    /// New-document form. For each, the words a reader actually sees
    /// are read back off the painted frame and compared with the
    /// sentence the session answers that control's own operation with.
    ///
    /// **Reading the paint is what makes this a row about the
    /// MAPPING.** A row that asserts a refusal value against the op
    /// that raises it holds the coupling and nothing else: every such
    /// assertion survives a permutation of the direction table in
    /// `toolbar_ui`, which is the map this file owns, or a swap of
    /// both arms of the direction-to-verb match. Both of those put the
    /// wrong sentence under the right button, which is the whole
    /// failure the unit exists to prevent, and only the frame can see
    /// it.
    ///
    /// **Three controls, hand-listed, and that is this row's blind
    /// spot.** A toolbar's disabled controls are not enumerable, so a
    /// fourth control
    /// added to `toolbar_ui` is not caught by this row. The instrument
    /// that finds one is the `add_enabled` sweep
    /// (`crates/viewer/README.md`, the disabled-control rule), and the
    /// gate that would hold the population rather than a reader's
    /// attention is filed as
    /// `work/unread/viewer-disabled-control-population-has-no-gate`.
    #[test]
    fn a_disabled_toolbar_control_says_what_its_own_operation_refuses() {
        let mut toolbar = Toolbar::open();
        // The New-document form is a draft the New… button opens; the
        // commit button is not drawn until it is open.
        toolbar.app.drafts.new_doc_name = Some(String::new());

        // A freshly assembled session is at the root of a history with
        // no branch beyond it, so all three are refused at once.
        for (label, op) in [
            ("Undo", SessionOp::Undo),
            ("Redo", SessionOp::Redo),
            (
                super::NEW_DOCUMENT_COMMIT,
                SessionOp::NewDocument {
                    name: String::new(),
                },
            ),
        ] {
            let shown = toolbar.reason_shown_on(label).unwrap_or_else(|| {
                panic!("{label} is disabled in this state, so hovering it says why")
            });
            // Refused, so the session is not moved by asking.
            let refused = toolbar
                .app
                .session
                .perform(op)
                .refusal
                .unwrap_or_else(|| panic!("{label}'s own operation refuses in this state"));
            assert_eq!(
                shown,
                refused.to_string(),
                "{label}: the words the reader sees are the words its own operation refuses with"
            );
        }

        // **And the history sentences name THEIR OWN direction.**
        //
        // Everything above compares two renderings of ONE value, so it
        // is true under any permutation of the direction-to-verb map:
        // swap both arms and each button says the opposite of the
        // truth with every assertion still passing. The two history
        // sentences differ by one verb and nothing else, so the verb
        // is the whole of what a reader gets, and it needs a pin that
        // does not route through the same `Display` both sides of an
        // equality do.
        //
        // The control's OWN NAME is that pin, and it is not a golden:
        // it holds a relation between two things the reader has in
        // front of them — what the button is called, and the reason it
        // gives — rather than a copy of the sentence. Re-wording the
        // refusal moves nothing here; putting the wrong verb under a
        // button reds it.
        //
        // The commit button takes no such clause: it has no sibling to
        // be confused with, and its sentence is about the name rather
        // than about the verb on the button.
        for (label, other) in [("Undo", "Redo"), ("Redo", "Undo")] {
            let shown = toolbar
                .reason_shown_on(label)
                .unwrap_or_else(|| panic!("{label} is disabled in this state"));
            let own = label.to_lowercase();
            let sibling = other.to_lowercase();
            assert!(
                shown.contains(&own),
                "the {label} button's reason does not say {own}: {shown:?}"
            );
            assert!(
                !shown.contains(&sibling),
                "the {label} button's reason is about {sibling}: {shown:?}"
            );
        }
    }

    /// **A dialog with no backend to open it says why on its own
    /// control**, in the words `ChooserBackend::unusable` answers —
    /// and a dialog that can open is drawn live and says nothing.
    ///
    /// Read off the painted frame, because the words a reader sees
    /// exist nowhere else (`painted_text`): a row over
    /// `ChooserBackend::unusable` alone holds the value and not that
    /// the two controls gate on it and show it. That the gate and the
    /// reason come from the same answer is held HERE, not by the type:
    /// a call site can still split them, and this row is what reds.
    /// Each backend is planted on the field the toolbar reads, so this
    /// holds whatever the test box's `PATH` and session bus are.
    #[test]
    fn a_dialog_with_no_backend_to_open_it_says_why_on_its_own_control() {
        use crate::platform::ChooserBackend;
        let mut toolbar = Toolbar::open();
        for backend in [
            ChooserBackend::ZenityPresent,
            ChooserBackend::PortalPossible,
            ChooserBackend::Absent,
        ] {
            toolbar.app.chooser = backend;
            for label in ["Open…", "Save As…"] {
                assert_eq!(
                    toolbar.drawn_live(label),
                    backend.unusable().is_none(),
                    "{label} under {backend:?}: live exactly when the backend is usable"
                );
                assert_eq!(
                    toolbar.reason_shown_on(label).as_deref(),
                    backend.unusable(),
                    "{label} under {backend:?}: the hover says the backend's reason, and only \
                     when it has one"
                );
            }
        }
    }

    /// **And the STATUS LINE wraps under itself**, in the real
    /// toolbar, which is Ev's second symptom measured where he saw it:
    /// egui starts a label in a wrapping row beside the widget before
    /// it and puts every line after the first at the panel's left
    /// edge. So the reading that answers the symptom is not "it fits"
    /// — it is that the lines begin under EACH OTHER.
    #[test]
    fn the_toolbars_status_line_wraps_under_itself_rather_than_at_the_windows_edge() {
        let row = toolbar_with(NARROW, Some(STATUS));
        assert!(
            row.status.len() > 1,
            "the fixture has to be longer than what is left of a {NARROW}-point \
             toolbar row, or this row is not about wrapping: {:?}",
            row.status
        );
        let first = row.status[0].left();
        let drift = row
            .status
            .iter()
            .map(|line| (line.left() - first).abs())
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(
            drift <= SLACK,
            "every line of the status begins under the first ({drift} points \
             of drift across {:?})",
            row.status
        );
        // **And the width it wrapped at is the WINDOW's**, which is
        // the reading that says the rows above can fail: a wrap at a
        // constant, or at anything but the row it is in, would read
        // the same line count at both widths.
        let wide = toolbar_with(UNBOUNDED, Some(STATUS));
        assert_eq!(
            wide.status.len(),
            1,
            "a window nothing constrains lays the same status out on one line \
             ({:?})",
            wide.status
        );
    }

    /// **What of the status line is on screen lies inside the panel, at
    /// every width**, the ones below the message floor included; where
    /// the panel is as wide as the floor, the whole line does, so the
    /// row scrolls only below the floor. The separator above the line
    /// spans the panel and no further.
    ///
    /// The sweep runs from below the floor to past a half-tiled
    /// desktop, in steps fine enough to put the controls' last line at
    /// every position the status line could follow it from.
    #[test]
    fn the_toolbars_status_line_is_drawn_inside_the_panel_at_every_width() {
        for width in (0..=26).map(|step| 120.0 + 20.0 * step as f32) {
            let row = toolbar_with(width, Some(STATUS));
            assert!(
                !row.status.is_empty(),
                "the status line was painted at a {width}-point window"
            );
            for line in &row.status {
                assert!(
                    (line.left() - row.panel.left()).abs() <= SLACK,
                    "at a {width}-point window every line of the status begins at the \
                     panel's left edge ({line:?}, panel {:?})",
                    row.panel
                );
                let shown = line.right().min(row.status_clip.right());
                assert!(
                    shown <= row.panel.right() + SLACK,
                    "at a {width}-point window the status line is on screen {} points past \
                     the panel ({line:?} under clip {:?}, panel {:?})",
                    shown - row.panel.right(),
                    row.status_clip,
                    row.panel
                );
                // Unclipped: a line laid out past the panel and hidden
                // by the scroll area's clip passes the reading above.
                if row.panel.width() >= row.floor {
                    assert!(
                        line.right() <= row.panel.right() + SLACK,
                        "at a {width}-point window, as wide as the floor ({}), the status \
                         line is laid out {} points past the panel, so it scrolls where it \
                         could wrap ({line:?}, panel {:?})",
                        row.floor,
                        line.right() - row.panel.right(),
                        row.panel
                    );
                }
            }
            assert!(
                row.rule_right.is_finite() && row.rule_right <= row.panel.right() + SLACK,
                "at a {width}-point window the status row's separator ends at {} against a \
                 panel ending at {}",
                row.rule_right,
                row.panel.right()
            );
        }
    }

    /// **Below the floor the status line scrolls**: it is laid out at
    /// the floor, wider than the panel, and scrolling it to its end
    /// brings that end on screen.
    #[test]
    fn below_the_floor_the_toolbars_status_line_scrolls_to_the_rest() {
        let width = 150.0;
        let at_rest = toolbar_with(width, Some(STATUS));
        let widest = at_rest
            .status
            .iter()
            .map(|line| line.right())
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(
            widest > at_rest.panel.right() + SLACK,
            "at a {width}-point window the status is laid out wider than the panel, at the \
             floor ({widest} against panel {:?})",
            at_rest.panel
        );
        let mut step = 0;
        let scrolled = toolbar_driven(
            width,
            |app| {
                app.status = Some(crate::frame::Message::new(
                    crate::frame::Subject::Document,
                    STATUS,
                    crate::frame::Retold::Again,
                ));
            },
            Some(STATUS),
            |row| {
                let Some(first) = row.status.first() else {
                    return Vec::new();
                };
                // A sideways wheel over the line, then frames for
                // egui's smoothed scroll to spend it in.
                let over = egui::pos2(row.panel.center().x, first.center().y);
                step += 1;
                match step {
                    1 => vec![
                        egui::Event::PointerMoved(over),
                        egui::Event::MouseWheel {
                            unit: egui::MouseWheelUnit::Point,
                            delta: egui::vec2(-400.0, 0.0),
                            phase: egui::TouchPhase::Move,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ],
                    2..=40 => vec![egui::Event::PointerMoved(over)],
                    _ => Vec::new(),
                }
            },
        );
        let end = scrolled
            .status
            .iter()
            .map(|line| line.right())
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(
            end <= scrolled.status_clip.right() + SLACK,
            "scrolled to its end, the status line's end is on screen ({end} against a clip \
             ending at {}; at rest {:?}, scrolled {:?})",
            scrolled.status_clip.right(),
            at_rest.status,
            scrolled.status
        );
    }

    /// **A refused save beside the original leaves its choice on the
    /// toolbar**, in the refusal's own file names, and the save that
    /// answers it takes it down.
    #[test]
    fn a_save_beside_the_original_puts_its_choice_on_the_toolbar() {
        let dir = std::env::temp_dir().join(format!("save-choice-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let (original, copy) = (dir.join("a.pncad"), dir.join("b.pncad"));
        let row = toolbar_drawn(
            NARROW,
            |app| {
                app.perform_batch(vec![SessionOp::Save(original.clone())]);
                assert!(
                    app.save_choice.is_none(),
                    "a save that lands offers nothing"
                );
                app.perform_batch(vec![SessionOp::Save(copy.clone())]);
            },
            None,
        );
        for label in [
            "Save over a.pncad",
            "Save b.pncad as a new document",
            "Dismiss",
        ] {
            assert!(
                row.painted.iter().any(|text| text == label),
                "the toolbar offers {label:?}: {:?}",
                row.painted
            );
        }
        // A second app over the original's document, so the same id.
        let ctx = egui::Context::default();
        let mut app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
            .expect("startup that needs no graphics device");
        app.perform_batch(vec![SessionOp::Open(original.clone())]);
        app.perform_batch(vec![SessionOp::Save(copy.clone())]);
        let standing = app.save_choice.clone();
        let fork = standing.as_ref().map(SaveChoice::fork);
        // A refusal that answers nothing leaves the choice standing.
        app.perform_batch(vec![SessionOp::SaveAsNewDocument(dir.join("notes.txt"))]);
        assert_eq!(app.save_choice, standing, "a refused fork answers nothing");
        app.perform_batch(vec![fork.expect("the refusal left its choice standing")]);
        let removed = std::fs::remove_dir_all(&dir);
        assert!(app.save_choice.is_none(), "the fork answered the choice");
        assert_eq!(app.session.path(), Some(copy.as_path()));
        removed.expect("the fixture removes its folder");
    }

    /// **Two startups are two documents**: the startup plate mints its
    /// id, so the second launch's plate, saved at the first's file,
    /// refuses that it would replace another document.
    #[test]
    fn two_startups_mint_two_ids_and_the_second_refuses_the_firsts_file() {
        use pncad::workspace::WorkspaceError;
        let dir = std::env::temp_dir().join(format!("two-startups-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let plate = dir.join("plate.pncad");
        let start = || {
            ViewerApp::assemble(&egui::Context::default(), pncad::tolerance::witness())
                .expect("startup that needs no graphics device")
        };
        let (mut first, mut second) = (start(), start());
        let first_id = first.session.committed_doc().id();
        assert_ne!(
            first_id,
            second.session.committed_doc().id(),
            "one startup plate, two identities"
        );
        let saved = first.session.perform(SessionOp::Save(plate.clone()));
        assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
        let refused = second
            .session
            .perform(SessionOp::Save(plate.clone()))
            .refusal;
        let removed = std::fs::remove_dir_all(&dir);
        assert!(
            matches!(
                refused,
                Some(crate::session::Refusal::Workspace(ref error))
                    if matches!(**error, WorkspaceError::SaveWouldReplaceDocument { id, .. } if id == first_id)
            ),
            "{refused:?}"
        );
        removed.expect("the fixture removes its folder");
    }

    /// **A typed name gains the document extension** the store reads,
    /// and one that has it is left alone.
    #[test]
    fn a_chosen_save_path_gains_the_document_extension() {
        use std::path::PathBuf;
        assert_eq!(
            super::with_doc_extension(PathBuf::from("/d/part")),
            PathBuf::from("/d/part.pncad")
        );
        assert_eq!(
            super::with_doc_extension(PathBuf::from("/d/part.v2")),
            PathBuf::from("/d/part.v2.pncad")
        );
        assert_eq!(
            super::with_doc_extension(PathBuf::from("/d/part.pncad")),
            PathBuf::from("/d/part.pncad")
        );
    }

    /// **The document's name is cut at the panel's edge**: the user
    /// chose it, so it can be wider than any window.
    #[test]
    fn a_long_document_name_is_truncated_inside_the_panel() {
        let name = "a document name its author made long enough to run past any narrow toolbar";
        // A folder of its own: a save scans the folder it writes into.
        let dir = std::env::temp_dir().join(format!("long-name-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let path = dir.join(format!("{name}.pncad"));
        let shown = super::document_name(Some(&path));
        let row = toolbar_drawn(
            NARROW,
            |app| {
                let outcome = app.session.perform(SessionOp::Save(path.clone()));
                // The name is read off the session's path, not the
                // file, so the file goes before anything can fail.
                let removed = std::fs::remove_dir_all(&dir);
                assert!(outcome.refusal.is_none(), "the fixture saves: {outcome:?}");
                removed.expect("the fixture removes the folder it saved into");
            },
            Some(&shown),
        );
        assert_eq!(
            row.status.len(),
            1,
            "the name is one line: {:?}",
            row.status
        );
        assert!(
            row.status[0].right() <= row.panel.right() + SLACK,
            "the name ends inside the panel ({:?}, panel {:?})",
            row.status[0],
            row.panel
        );
    }
    /// The context startup installed onto, and the app it assembled.
    ///
    /// [`ViewerApp::assemble`] is the half of startup that needs no
    /// graphics device, and every context-wide install is in
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

    /// **Startup turns the toolkit's wheel-zoom routing off**, so a
    /// ctrl/⌘+wheel reaches the viewport as a scroll
    /// (`pane::viewport::configure_input`). The first assertion is
    /// what makes the second a statement about startup: an untouched
    /// context routes a COMMAND wheel to zoom.
    #[test]
    fn startup_reads_every_wheel_as_a_scroll() {
        let untouched =
            egui::Context::default().options(|options| options.input_options.zoom_modifier);
        assert_ne!(
            untouched,
            egui::Modifiers::NONE,
            "the toolkit's default already reads every wheel as a scroll, so nothing below sees the install"
        );
        let (ctx, _app) = started();
        assert_eq!(
            ctx.options(|options| options.input_options.zoom_modifier),
            egui::Modifiers::NONE,
            "startup leaves a modified wheel routed to the toolkit's zoom"
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

/// **What the properties pane says about a selection that no longer
/// denotes**, read off a whole headless frame of the real app — so the
/// pane's own gates and deletions are held, not only the free function
/// that draws the verdict (`pane::properties::verdict_tests`).
#[cfg(test)]
mod properties_pane_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used, clippy::panic)]

    use eframe::egui;
    use pncad::document::{Axis3, Node, RecipeNodeId, SlotId};

    use super::{DocumentDerived, ViewerApp};
    use crate::session::{Selection, SessionOp};

    /// The window every whole-app row here is drawn in unless it needs
    /// a taller one.
    pub(super) const WINDOW: egui::Vec2 = egui::vec2(1600.0, 1200.0);

    pub(super) use super::drive::{app_frame, index_answered, quiescent, settle};

    /// **The wait answers a frame that paints nothing outstanding.**
    /// The startup evaluation, the display fit and the index build
    /// each run on a worker, and the toolbar says each while it runs.
    /// A wait that stopped at the landing answers the landing frame,
    /// whose fit was submitted in that same frame, so it paints
    /// `indexing…`; one that stopped after a fixed count paints
    /// whichever phase the scheduler reached.
    #[test]
    fn the_wait_answers_a_frame_with_nothing_outstanding() {
        let ctx = egui::Context::default();
        let mut app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
            .expect("startup that needs no graphics device");
        let mut frame = eframe::Frame::_new_kittest();
        let painted: Vec<String> = settle(&mut app, quiescent, |app| {
            app_frame(&ctx, app, &mut frame, WINDOW, None, Vec::new())
                .into_iter()
                .map(|landed| landed.text)
                .collect()
        });
        for word in ["evaluating…", "Cancel", "indexing…"] {
            assert!(
                !painted.iter().any(|text| text == word),
                "{word:?} in a settled frame: {painted:?}"
            );
        }
    }

    /// Every text the app painted with `selection` made, once it has
    /// settled ([`Driven::quiet`]), in paint order.
    fn painted_with(selection: Selection) -> Vec<String> {
        Driven::with(vec![SessionOp::Select(selection)])
            .quiet()
            .into_iter()
            .map(|(text, _)| text)
            .collect()
    }

    /// **The collapsed Properties section whose body paints `kind`'s
    /// activation button**. The one answer every row here that opens a
    /// tool reads.
    fn section_of(kind: crate::tools::ToolKind) -> &'static str {
        use crate::pane::create::{ADD_FEATURE, ASSEMBLE_PARTS, BLEND_EDGES, COMBINE_BODIES};
        use crate::tools::ToolKind;
        match kind {
            ToolKind::Mate => ASSEMBLE_PARTS,
            ToolKind::Revolve => ADD_FEATURE,
            ToolKind::Boolean
            | ToolKind::Split
            | ToolKind::Transform
            | ToolKind::Pattern
            | ToolKind::Part
            | ToolKind::Duplicate
            | ToolKind::Mirror => COMBINE_BODIES,
            ToolKind::Blend => BLEND_EDGES,
        }
    }

    /// Every text the app painted once the startup document has
    /// LANDED, with `tool` open and `picks` fed to it as the frame
    /// feeds a click's selection (`Tools::feed`, then the batch), and
    /// the collapsed section that hosts it ([`section_of`]) clicked
    /// open.
    ///
    /// Landed first, because the survival step (`sync_scene`'s
    /// `Tools::reconcile`) only asks a face pick whether it resolves
    /// once there is a landed pair to ask — a panel read before that
    /// would show picks the next frame might drop.
    fn painted_with_tool(
        tool: crate::tools::ToolKind,
        picks: impl FnOnce(RecipeNodeId) -> Vec<Selection>,
    ) -> (RecipeNodeId, Vec<String>) {
        let ctx = egui::Context::default();
        let mut app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
            .expect("startup that needs no graphics device");
        let mut frame = eframe::Frame::_new_kittest();
        let mut paint = |app: &mut ViewerApp, events: Vec<egui::Event>| {
            app_frame(&ctx, app, &mut frame, WINDOW, None, events)
        };
        // The startup document's body is its extrude (`plate_with_hole`).
        let body = extrude();
        app.tools.open(tool);
        clicked(&mut app, picks(body));
        let landed = settle(&mut app, quiescent, |app| paint(app, Vec::new()));
        {
            let section = section_of(tool);
            let at = landed
                .iter()
                .find(|landed| landed.text == section)
                .map(|landed| landed.allocated.center())
                .expect("the section header is painted");
            let button = |pressed| egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            paint(
                &mut app,
                vec![egui::Event::PointerMoved(at), button(true), button(false)],
            );
        }
        // Two frames more: a section's open animation and a survival
        // step that drops the pick both settle within them.
        paint(&mut app, Vec::new());
        let texts = paint(&mut app, Vec::new())
            .into_iter()
            .map(|landed| landed.text)
            .collect();
        (body, texts)
    }

    /// **A seated tool's panel, painted by the app**: the held pick's
    /// line, in the tool's role order, reaching the Properties pane
    /// through the whole frame — survival step included.
    #[test]
    fn a_seated_tool_panel_shows_its_held_picks() {
        let (body, painted) = painted_with_tool(crate::tools::ToolKind::Split, |body| {
            vec![Selection::Node(body)]
        });
        // The startup body is an extrude, spoken by its kind and tag.
        let line = format!(
            "split target: Extrude {}",
            test_utils::refusal::tag(body.0.digest())
        );
        assert!(painted.contains(&line), "{line:?} in {painted:?}");
    }

    /// **The boolean panel, painted by the app**: a union's held member
    /// in its list, reaching the Properties pane through the whole
    /// frame.
    #[test]
    fn the_boolean_panel_lists_its_held_members() {
        let (body, painted) = painted_with_tool(crate::tools::ToolKind::Boolean, |body| {
            vec![Selection::Node(body), Selection::Node(body)]
        });
        let tag = test_utils::refusal::tag(body.0.digest());
        let line = format!("member 1: Extrude {tag}; member 2: Extrude {tag}");
        assert!(painted.contains(&line), "{line:?} in {painted:?}");
    }

    /// **The mate panel, painted by the app**: a face pick on the
    /// startup body survives the landed survival step and is said the
    /// way the seated panels say a pick.
    #[test]
    fn the_mate_panel_shows_its_held_picks() {
        let (_, painted) = painted_with_tool(crate::tools::ToolKind::Mate, |_| {
            vec![Selection::Face(cap_of(pncad::prelude::CapEnd::End))]
        });
        // A pick is the face in its words, on the copy clicked.
        let line = format!(
            "on: the end cap of Extrude {}, on Place {}; to: none picked",
            test_utils::refusal::tag(extrude().0.digest()),
            test_utils::refusal::tag(placement().0.digest())
        );
        assert!(painted.contains(&line), "{line:?} in {painted:?}");
    }

    /// **A deleted node is called deleted, and nothing claims it
    /// carries no parameters** — it carries nothing because it is not
    /// there, which the `live()` gate on that line keeps unsaid.
    #[test]
    fn a_deleted_node_is_not_said_to_carry_no_parameters() {
        let painted = painted_with(Selection::Node(RecipeNodeId::new(0, 999)));
        assert!(painted.iter().any(|text| text == "deleted"), "{painted:?}");
        assert!(
            !painted
                .iter()
                .any(|text| text.contains("carries no parameters")),
            "{painted:?}"
        );
    }

    /// **An undeclared variable is said once**: against the frame with
    /// nothing selected, the pane gains the verdict line and loses the
    /// "select a feature" prompt, and nothing else.
    #[test]
    fn an_undeclared_variable_is_said_once_in_the_pane() {
        let var = pncad::document::VarId::new(0, 0x0123_4567_89ab_cdef);
        let verdict = crate::session::Refusal::undeclared_wording(
            &pncad::document::SpokenVar::new(var, None),
        );
        let mut with = painted_with(Selection::Variable(var));
        let mut without = painted_with(Selection::None);
        assert!(
            without.iter().any(|text| text == "select a feature"),
            "the properties pane is drawn in this frame: {without:?}"
        );
        without.retain(|text| text != "select a feature");
        without.push(verdict);
        with.sort();
        without.sort();
        assert_eq!(with, without);
    }

    /// The whole app in a headless context, driven frame by frame with
    /// a pointer: for a Properties control whose words exist only on
    /// hover, and whose click has to reach the session.
    struct Driven {
        ctx: egui::Context,
        app: ViewerApp,
        frame: eframe::Frame,
        /// Frames are numbered in seconds because a tooltip is a
        /// function of how long the pointer has been still.
        time: f64,
    }

    impl Driven {
        /// A point inside the window that no control of the
        /// Properties pane sits under.
        const ELSEWHERE: egui::Pos2 = egui::Pos2::new(1590.0, 990.0);

        /// The startup document with `ops` performed, a node selected
        /// among them, and one frame drawn.
        fn with(ops: Vec<SessionOp>) -> Self {
            Self::with_seams(ops, |_| {})
        }

        /// [`Self::with`], over the seams `seams` puts in first.
        fn with_seams(ops: Vec<SessionOp>, seams: impl FnOnce(&mut ViewerApp)) -> Self {
            let ctx = egui::Context::default();
            // A tooltip this row waited for would be a row about
            // `tooltip_delay`.
            ctx.all_styles_mut(|style| style.interaction.tooltip_delay = 0.0);
            let mut app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
                .expect("startup that needs no graphics device");
            seams(&mut app);
            app.perform_batch(ops);
            let mut driven = Self {
                ctx,
                app,
                frame: eframe::Frame::_new_kittest(),
                time: 0.0,
            };
            driven.frame(vec![egui::Event::PointerMoved(Self::ELSEWHERE)]);
            driven
        }

        /// One frame of the real app with `events` delivered.
        fn frame(&mut self, events: Vec<egui::Event>) -> Vec<(String, egui::Rect)> {
            let Self {
                ctx,
                app,
                frame,
                time,
            } = self;
            Self::drawn(ctx, app, frame, time, events)
        }

        /// [`Self::frame`]'s body, over the fields [`Self::settled`]
        /// holds apart.
        fn drawn(
            ctx: &egui::Context,
            app: &mut ViewerApp,
            frame: &mut eframe::Frame,
            time: &mut f64,
            events: Vec<egui::Event>,
        ) -> Vec<(String, egui::Rect)> {
            *time += 1.0;
            app_frame(ctx, app, frame, WINDOW, Some(*time), events)
                .into_iter()
                .map(|landed| (landed.text, landed.allocated))
                .collect()
        }

        /// Frames until `until` holds ([`settle`]), each moving the
        /// pointer to `pointer` if there is one; answers the settled
        /// frame.
        fn settled(
            &mut self,
            until: impl Fn(&ViewerApp) -> bool,
            pointer: Option<egui::Pos2>,
        ) -> Vec<(String, egui::Rect)> {
            let Self {
                ctx,
                app,
                frame,
                time,
            } = self;
            settle(app, until, |app| {
                let events = pointer.map(egui::Event::PointerMoved).into_iter().collect();
                Self::drawn(ctx, app, frame, time, events)
            })
        }

        /// Quiet frames until the app is [`quiescent`].
        fn settle(&mut self) {
            self.settled(quiescent, Some(Self::ELSEWHERE));
        }

        /// The app once [`quiescent`], in two frames with the
        /// pointer parked away from everything: the second is what the
        /// app draws when nothing is hovered.
        fn quiet(&mut self) -> Vec<(String, egui::Rect)> {
            self.settle();
            self.frame(vec![egui::Event::PointerMoved(Self::ELSEWHERE)]);
            self.frame(vec![egui::Event::PointerMoved(Self::ELSEWHERE)])
        }

        /// Two frames with the pointer resting on `at`: egui decides
        /// hover against the rect the previous frame left behind.
        fn hovered(&mut self, at: egui::Pos2) -> Vec<(String, egui::Rect)> {
            self.frame(vec![egui::Event::PointerMoved(at)]);
            self.frame(vec![egui::Event::PointerMoved(at)])
        }

        /// The runs `hovered` painted that `quiet` did not.
        fn gained(
            quiet: &[(String, egui::Rect)],
            hovered: Vec<(String, egui::Rect)>,
        ) -> Vec<String> {
            hovered
                .into_iter()
                .map(|(run, _)| run)
                .filter(|run| !quiet.iter().any(|(before, _)| before == run))
                .collect()
        }

        /// Where the ONE text run reading exactly `text` was painted.
        fn only(painted: &[(String, egui::Rect)], text: &str) -> egui::Pos2 {
            let hits: Vec<egui::Rect> = painted
                .iter()
                .filter(|(run, _)| run == text)
                .map(|(_, rect)| *rect)
                .collect();
            assert_eq!(
                hits.len(),
                1,
                "exactly one run reads {text:?}, so the pointer can be put on it: {painted:?}"
            );
            hits[0].center()
        }

        /// **What resting the pointer on `text` adds to the picture** —
        /// the difference between a quiet frame and a hovered one, so
        /// it names no wording of its own.
        fn gained_hovering(&mut self, text: &str) -> Vec<String> {
            let quiet = self.quiet();
            let at = Self::only(&quiet, text);
            let hovered = self.hovered(at);
            Self::gained(&quiet, hovered)
        }

        /// Where each text run reading exactly `text` was painted on a
        /// quiet frame, in paint order.
        fn each(&mut self, text: &str) -> Vec<egui::Pos2> {
            self.quiet()
                .into_iter()
                .filter(|(run, _)| run == text)
                .map(|(_, rect)| rect.center())
                .collect()
        }

        /// Click the one run reading `text`, found on the next frame,
        /// then draw a frame with the pointer still on it.
        ///
        /// It does not wait: rows that hold a run open on purpose
        /// ([`gated`]) click through it, so a row that needs the
        /// picture settled first settles first.
        fn click(&mut self, text: &str) {
            let painted = self.frame(Vec::new());
            let at = Self::only(&painted, text);
            self.click_at(at);
        }

        /// Click at `at`, then draw a frame with the pointer still on
        /// it.
        fn click_at(&mut self, at: egui::Pos2) {
            let press = |pressed| egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            self.frame(vec![egui::Event::PointerMoved(at)]);
            self.frame(vec![press(true), press(false)]);
            self.frame(Vec::new());
        }

        /// One quiet frame once the app is [`quiescent`], with where
        /// each run landed and the clip it was painted under.
        fn landed(&mut self) -> Vec<crate::pane::headless::Landed> {
            self.settle();
            self.time += 1.0;
            let time = Some(self.time);
            let Self {
                ctx, app, frame, ..
            } = self;
            app_frame(
                ctx,
                app,
                frame,
                WINDOW,
                time,
                vec![egui::Event::PointerMoved(Self::ELSEWHERE)],
            )
        }

        /// One slot's row, as the document now holds it.
        fn row(&self, node: RecipeNodeId, slot: SlotId) -> crate::props::SlotRow {
            crate::props::slot_rows(self.app.session.doc(), node)
                .into_iter()
                .find(|row| row.slot == slot)
                .expect("the node lists the slot")
        }
    }

    /// The startup plate's node at `at` in document order, read off the
    /// startup document itself (`scene::plate_with_hole`): its profile,
    /// its extrude, its placement.
    fn startup_node(at: usize) -> RecipeNodeId {
        let (doc, _) = crate::scene::plate_with_hole(pncad::tolerance::witness())
            .expect("the startup document");
        doc.ids()[at]
    }

    /// The startup plate's extrude, and its one slot.
    fn extrude() -> RecipeNodeId {
        startup_node(1)
    }

    /// The startup plate's world placement: the node its drawn copy is
    /// picked on.
    fn placement() -> RecipeNodeId {
        startup_node(2)
    }

    /// **The startup document with a revolve added, then `then`'s ops**:
    /// a circle off its profile's own y axis turned a full turn about
    /// it. The revolve's axis point is a vector (`SlotId::Origin`, its x
    /// and y), which is what the vector rows below are about. Answers
    /// the pane and the revolve.
    fn with_revolve(then: impl FnOnce(RecipeNodeId) -> Vec<SessionOp>) -> (Driven, RecipeNodeId) {
        let mut pane = Driven::with(vec![SessionOp::AddProfile {
            loops: vec![
                pncad::document::LoopProgram::circle(0.02, 0.0, 0.005).expect("a finite circle"),
            ],
        }]);
        let of_kind = |pane: &Driven, noun: &str| {
            let doc = pane.app.session.committed_doc();
            doc.ids()
                .into_iter()
                .filter(|&id| {
                    doc.node(id)
                        .is_some_and(|node| pncad::document::node_kind_noun(node) == noun)
                })
                .collect::<Vec<_>>()
        };
        let profile = *of_kind(&pane, "Profile")
            .last()
            .expect("the circle was added");
        // Written, non-zero numbers: a lone `0` is the constant, which
        // reads no variable and so has no unit a picker could write.
        let at = || crate::test_support::len(0.001);
        pane.app.perform_batch(vec![SessionOp::AddRevolve {
            profile,
            axis_through: [at(), at()],
            axis_heading: pncad::document::Formula::turn_over(4).expect("a quarter turn"),
            angle: crate::test_support::ang(core::f64::consts::TAU),
        }]);
        let revolve = *of_kind(&pane, "Revolve")
            .first()
            .expect("the revolve was added");
        pane.app.perform_batch(then(revolve));
        pane.quiet();
        (pane, revolve)
    }

    /// An expression with no variable in it: computed all the same,
    /// so the slot it drives has no written unit.
    const COMPUTED: &str = "1 mm + 1 mm";

    /// The sentence `SetSlotUnit` refuses a computed slot with, as the
    /// session renders it — read from the op's own admission.
    fn refusal_for(pane: &Driven, node: RecipeNodeId, slot: SlotId) -> String {
        pane.app
            .session
            .slot_unit_refusal(node, slot)
            .expect("SetSlotUnit refuses a computed slot")
            .to_string()
    }

    /// Whatever the status line holds after the frames drawn so far.
    fn status(pane: &Driven) -> Option<String> {
        pane.app
            .status
            .as_ref()
            .map(|message| message.text().to_owned())
    }

    /// The startup extrude's depth typed as `value`, after a length
    /// variable `name` declared at the same value; the extrude selected.
    fn typed_beside(name: &'static str, value: f64) -> Vec<SessionOp> {
        vec![
            SessionOp::DeclareVar {
                name: pncad::document::VarName::from_static(name),
                value: pncad::document::FreeVar::continuous(
                    pncad::document::Dimension::Length,
                    value,
                ),
            },
            SessionOp::SetSlot {
                node: extrude(),
                slot: SlotId::Distance,
                value: crate::props::SlotValue::Continuous(value),
            },
            SessionOp::Select(Selection::Node(extrude())),
        ]
    }

    /// **The offer is drawn under the typed slot, and its button writes
    /// the slot to read the variable** — the real pane, the click
    /// reaching the session as `SetSlotVariable`. The variable list
    /// says `beam` too, so the button is the run on the offer's line.
    #[test]
    fn the_offer_is_drawn_and_its_button_makes_the_slot_read_the_variable() {
        let mut pane = Driven::with(typed_beside("beam", 0.004));
        let painted = pane.quiet();
        let line = Driven::only(&painted, "same value as");
        let at = painted
            .iter()
            .filter(|(run, _)| run == "beam")
            .map(|(_, rect)| rect.center())
            .min_by(|p, q| (p.y - line.y).abs().total_cmp(&(q.y - line.y).abs()))
            .expect("the offered variable is drawn");
        assert!((at.y - line.y).abs() < 4.0, "on the offer's line");
        let press = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        pane.frame(vec![egui::Event::PointerMoved(at)]);
        pane.frame(vec![press(true), press(false)]);
        pane.frame(Vec::new());
        let doc = pane.app.session.committed_doc();
        assert_eq!(
            doc.slot(extrude(), SlotId::Distance),
            doc.var_named("beam"),
            "the slot reads the variable offered"
        );
        let after = pane.quiet();
        assert!(
            !after.iter().any(|(run, _)| run == "same value as"),
            "an accepted offer is no longer drawn"
        );
    }

    /// **Declining is a button too**, and it leaves the document where
    /// it was: the slot keeps the variable its typed value minted.
    #[test]
    fn keep_separate_declines_the_offer_and_moves_no_document() {
        let mut pane = Driven::with(typed_beside("beam", 0.004));
        pane.quiet();
        let minted = pane
            .app
            .session
            .committed_doc()
            .slot(extrude(), SlotId::Distance);
        let steps = pane.app.session.history().len();
        pane.click("keep separate");
        let after = pane.quiet();
        assert!(!after.iter().any(|(run, _)| run == "same value as"));
        assert_eq!(pane.app.session.history().len(), steps, "no undo step");
        assert_eq!(
            pane.app
                .session
                .committed_doc()
                .slot(extrude(), SlotId::Distance),
            minted
        );
    }

    /// **The naming field opens empty and stores nothing until a name
    /// is committed** (VR2; Ev, PR 4247: no proposal): the field holds
    /// no text, its commit is disabled while it names nothing, the
    /// document holds no name while it is open, and the commit is one
    /// `RenameVar` of the variable the slot reads.
    #[test]
    fn the_naming_field_opens_empty_and_stores_only_on_commit() {
        let mut pane = Driven::with(vec![SessionOp::Select(Selection::Node(extrude()))]);
        pane.quiet();
        let var = pane
            .app
            .session
            .committed_doc()
            .slot(extrude(), SlotId::Distance)
            .expect("the extrude reads its depth");
        assert!(pane.app.session.committed_doc().var_name(var).is_none());
        pane.click("name…");
        assert_eq!(
            pane.app.drafts.name_draft,
            Some(crate::drafts::NameDraft {
                node: extrude(),
                slot: SlotId::Distance,
                var,
                share: false,
                text: String::new(),
            }),
            "the field opens empty, for the variable the slot reads"
        );
        pane.quiet();
        pane.click("Name");
        assert!(
            pane.app.session.committed_doc().var_name(var).is_none(),
            "an empty field names nothing"
        );
        pane.app
            .drafts
            .name_draft
            .as_mut()
            .expect("the field is still open")
            .text = "distance".to_owned();
        pane.quiet();
        assert!(
            pane.app.session.committed_doc().var_name(var).is_none(),
            "typed text stores nothing until committed"
        );
        pane.click("Name");
        let doc = pane.app.session.committed_doc();
        assert_eq!(
            doc.var_name(var).map(|name| name.as_str()),
            Some("distance")
        );
        assert_eq!(doc.slot(extrude(), SlotId::Distance), Some(var));
        assert!(pane.app.drafts.name_draft.is_none(), "the field closes");
    }

    /// **An open naming field names the variable it was opened for and
    /// no other**: accepting an offer while the field is open makes the
    /// slot read the named `beam`, and the field closes rather than
    /// rename `beam` through the slot.
    #[test]
    fn a_naming_field_closes_when_its_slot_reads_another_variable() {
        let mut pane = Driven::with(typed_beside("beam", 0.004));
        pane.quiet();
        pane.click("name\u{2026}");
        assert!(pane.app.drafts.name_draft.is_some(), "the field is open");
        let beam = pane
            .app
            .session
            .committed_doc()
            .var_named("beam")
            .expect("beam is declared");
        let accepted = pane.app.session.perform(SessionOp::SetSlotVariable {
            node: extrude(),
            slot: SlotId::Distance,
            var: beam,
            name: None,
        });
        assert!(accepted.refusal.is_none(), "{:?}", accepted.refusal);
        let painted = pane.quiet();
        assert!(pane.app.drafts.name_draft.is_none(), "the field closes");
        assert!(!painted.iter().any(|(run, _)| run == "Name"));
        assert_eq!(
            pane.app
                .session
                .committed_doc()
                .var_name(beam)
                .map(|name| name.as_str()),
            Some("beam"),
            "beam keeps its name"
        );
    }

    /// **Accepting an unnamed offer names it and shares it as one
    /// step** (VR2: a variable two slots share has a name): a revolve's
    /// axis point x and the extrude's depth are typed alike, so the depth is
    /// offered the origin's unnamed variable; its button opens the empty
    /// naming field rather than sharing it, the document moving not at
    /// all, and the commit names it and makes the depth read it in one
    /// undo step, which undo takes back whole.
    #[test]
    fn accepting_an_unnamed_offer_opens_the_name_field_and_shares_as_one_step() {
        let value = 0.004;
        let (mut pane, revolve) = with_revolve(|revolve| {
            vec![
                SessionOp::SetSlot {
                    node: revolve,
                    slot: SlotId::Origin(pncad::document::Axis3::X),
                    value: crate::props::SlotValue::Continuous(value),
                },
                SessionOp::SetSlot {
                    node: extrude(),
                    slot: SlotId::Distance,
                    value: crate::props::SlotValue::Continuous(value),
                },
                SessionOp::Select(Selection::Node(extrude())),
            ]
        });
        pane.quiet();
        let doc = pane.app.session.committed_doc();
        let origin = doc
            .slot(revolve, SlotId::Origin(pncad::document::Axis3::X))
            .expect("the revolve reads its axis point x");
        assert!(doc.var_name(origin).is_none(), "the premise: it is unnamed");
        let offered = pane.app.session.offered(extrude(), SlotId::Distance);
        let offer = offered
            .iter()
            .find(|offer| offer.var == origin)
            .expect("the origin's variable is offered")
            .label
            .clone();
        let steps = pane.app.session.history().len();
        pane.click(&offer);
        assert_eq!(
            pane.app.drafts.name_draft,
            Some(crate::drafts::NameDraft {
                node: extrude(),
                slot: SlotId::Distance,
                var: origin,
                share: true,
                text: String::new(),
            }),
            "the field opens empty, for the variable offered"
        );
        assert_eq!(pane.app.session.history().len(), steps, "nothing moved yet");
        pane.app
            .drafts
            .name_draft
            .as_mut()
            .expect("the field is open")
            .text = "gap".to_owned();
        pane.quiet();
        let prior = pane.app.session.committed_doc().clone();
        pane.click("Name");
        let doc = pane.app.session.committed_doc();
        assert_eq!(doc.var_name(origin).map(|name| name.as_str()), Some("gap"));
        assert_eq!(
            doc.slot(extrude(), SlotId::Distance),
            Some(origin),
            "the depth reads the named variable"
        );
        assert_eq!(pane.app.session.history().len(), steps + 1, "one step");
        assert!(pane.app.drafts.name_draft.is_none(), "the field closes");
        assert!(pane.app.session.perform(SessionOp::Undo).refusal.is_none());
        assert!(
            pane.app.session.committed_doc().bit_eq(&prior),
            "undo takes the name and the share back whole"
        );
    }

    /// **A name the door refuses leaves the field open with its text**:
    /// `distance` is taken, so `RenameVar` refuses, the variable stays
    /// unnamed, and the person's text is still there to amend.
    #[test]
    fn a_refused_name_keeps_the_field_and_its_text() {
        let mut pane = Driven::with(typed_beside("distance", 0.009));
        pane.quiet();
        pane.click("name\u{2026}");
        let draft = pane
            .app
            .drafts
            .name_draft
            .as_mut()
            .expect("the field is open");
        assert!(draft.text.is_empty(), "the field opens empty");
        draft.text = "distance".to_owned();
        let var = draft.var;
        pane.click("Name");
        assert!(pane.app.session.committed_doc().var_name(var).is_none());
        assert_eq!(
            pane.app
                .drafts
                .name_draft
                .as_ref()
                .map(|draft| draft.text.as_str()),
            Some("distance"),
            "the field and its text stay"
        );
    }

    /// **A driven slot's unit picker is drawn, cannot be opened, and
    /// says what `SetSlotUnit` would refuse with** — the whole app,
    /// the real pane, a slot made driven through the real door.
    ///
    /// The runtime value that makes it false is a picker gated on
    /// anything weaker than the op's admission: hovered, it would say
    /// nothing, and clicked it would open and offer a pick the op then
    /// refuses.
    #[test]
    fn a_driven_slots_unit_picker_is_disabled_with_the_refusal_it_would_get() {
        let mut pane = Driven::with(vec![
            SessionOp::SetSlotExpression {
                node: extrude(),
                slot: SlotId::Distance,
                text: COMPUTED.to_owned(),
            },
            SessionOp::Select(Selection::Node(extrude())),
        ]);
        let before = pane.row(extrude(), SlotId::Distance);
        assert!(before.driver.is_driven(), "the setup drove the slot");
        // No writable component, so no unit: the picker says what the
        // slot is instead.
        let gained = pane.gained_hovering("computed");
        let said = refusal_for(&pane, extrude(), SlotId::Distance);
        assert_eq!(
            gained,
            vec![said.clone()],
            "the hover is the op's own sentence"
        );
        // Planted, not only compared with the one home: the words a
        // reader gets for this row.
        assert_eq!(
            said,
            format!(
                "the distance slot on Extrude {} is computed, so it has no written unit to change \
                 — set an expression to change what it says",
                extrude()
            )
        );
        pane.click("computed");
        let after = pane.quiet();
        assert!(
            !after.iter().any(|(run, _)| run == "mm"),
            "a refused picker does not open: {after:?}"
        );
        assert_eq!(pane.row(extrude(), SlotId::Distance), before);
    }

    /// A seam that answers a request only on the poll after
    /// `hold` polls have come back empty: a run that lands a known
    /// number of frames late.
    struct Late<S> {
        inner: S,
        hold: usize,
        polls: usize,
    }

    impl<S> Late<S> {
        fn new(inner: S, hold: usize) -> Self {
            Self {
                inner,
                hold,
                polls: 0,
            }
        }

        /// Whether this poll is one the seam holds back.
        fn holds(&mut self) -> bool {
            self.polls += 1;
            self.polls <= self.hold
        }
    }

    impl crate::evalseam::EvalService for Late<crate::evalseam::InlineEvaluator> {
        fn submit(&mut self, request: crate::evalseam::EvalRequest) {
            self.polls = 0;
            self.inner.submit(request);
        }

        fn cancel(&mut self) {
            self.inner.cancel();
        }

        fn poll(&mut self) -> Option<crate::evalseam::EvalDone> {
            if self.holds() {
                None
            } else {
                self.inner.poll()
            }
        }

        fn busy(&self) -> bool {
            self.inner.busy()
        }
    }

    impl crate::evalseam::IndexService for Late<crate::evalseam::InlineIndexer> {
        fn submit(&mut self, request: crate::evalseam::IndexRequest) {
            self.polls = 0;
            self.inner.submit(request);
        }

        fn poll(&mut self) -> Option<crate::evalseam::IndexDone> {
            if self.holds() {
                None
            } else {
                self.inner.poll()
            }
        }

        fn busy(&self) -> bool {
            self.inner.busy()
        }
    }

    /// **The hover diff settles the app first.** The diff is of the
    /// whole app, so a run that lands between its quiet frame and its
    /// hovered one turns the toolbar's `evaluating…` into `indexing…`,
    /// and the new label reads as the hover's.
    ///
    /// Forced here: the startup evaluation is held for the frame
    /// [`Driven::with`] draws and the two a quiet read draws unsettled,
    /// and lands on the first hovered frame; the index build it asks
    /// for is held a frame more, so the second hovered frame is
    /// still indexing. Only [`Driven::quiet`]'s settle keeps it out.
    #[test]
    fn a_hover_diff_waits_for_a_run_that_lands_between_its_frames() {
        let mut pane =
            Driven::with_seams(vec![SessionOp::Select(Selection::Node(extrude()))], |app| {
                let tol = pncad::tolerance::witness();
                app.session = crate::session::DocSession::new(
                    app.session.doc().clone(),
                    tol,
                    Box::new(Late::new(crate::evalseam::InlineEvaluator::new(), 3)),
                );
                app.picks = crate::pickcache::PickCache::new(Box::new(Late::new(
                    crate::evalseam::InlineIndexer::new(),
                    1,
                )));
            });
        assert_eq!(
            pane.app.session.outstanding(),
            crate::session::Outstanding::Evaluating,
            "the startup run is still held after the first frame"
        );
        let gained = pane.gained_hovering("m");
        assert!(
            gained.is_empty(),
            "the diff holds only what the hover drew: {gained:?}"
        );
    }

    /// **The same drive on a literal slot opens the picker and the
    /// pick lands** — the row that keeps the one above from passing
    /// because the harness missed the combo, and that no sentence is
    /// owed where the op would accept.
    #[test]
    fn a_literal_slots_unit_picker_opens_and_says_nothing() {
        let mut pane = Driven::with(vec![SessionOp::Select(Selection::Node(extrude()))]);
        assert!(pane.gained_hovering("m").is_empty());
        pane.click("m");
        pane.click("mm");
        assert_eq!(
            pane.row(extrude(), SlotId::Distance).unit,
            Some(pncad::prelude::MM.def())
        );
    }

    /// **A driven slot's row stays inside the Properties pane however
    /// long its expression** — drawn by the real `slot_group_ui`. The
    /// field shows the value, and the source is said under the row,
    /// wrapped inside the pane.
    #[test]
    fn a_long_driven_source_stays_inside_the_properties_pane() {
        let source = vec!["1 mm"; 40].join(" + ");
        let mut pane = Driven::with(vec![
            SessionOp::SetSlotExpression {
                node: extrude(),
                slot: SlotId::Distance,
                text: source.clone(),
            },
            SessionOp::Select(Selection::Node(extrude())),
        ]);
        let row = pane.row(extrude(), SlotId::Distance);
        assert_eq!(row.source.as_deref(), Some(source.as_str()));
        let field = crate::props::field_text(&row, pane.app.session.notation());
        assert_eq!(field, "= 0.04 m");
        let quoted = format!("{} = {source}", SlotId::Distance.label());
        let landed = pane.landed();
        for text in [field.as_str(), quoted.as_str()] {
            let run = landed
                .iter()
                .find(|landed| landed.text == text)
                .unwrap_or_else(|| panic!("`{text}` was never painted"));
            for line in &run.rows {
                assert!(
                    line.right() <= run.clip.right() + crate::pane::headless::SLACK,
                    "`{text}` ends {} points past the pane",
                    line.right() - run.clip.right()
                );
            }
        }
        assert!(
            !landed.iter().any(|landed| landed.text == source),
            "the source is not the field's text"
        );
    }

    /// **Each component of a driven vector opens its edit on its OWN
    /// source**, and clicking away from it writes nothing — over a
    /// revolve's axis point, the vector a document holds.
    #[test]
    fn each_driven_vector_component_edits_its_own_source() {
        let axes = [Axis3::X, Axis3::Y];
        let sources = ["1.5 mm + 0.5 mm", "2.5 mm + 0.5 mm"];
        let (mut pane, pattern) = with_revolve(|_| Vec::new());
        let mut ops: Vec<SessionOp> = axes
            .iter()
            .zip(sources)
            .map(|(axis, source)| SessionOp::SetSlotExpression {
                node: pattern,
                slot: SlotId::Origin(*axis),
                text: source.to_owned(),
            })
            .collect();
        ops.push(SessionOp::Select(Selection::Node(pattern)));
        pane.app.perform_batch(ops);
        pane.quiet();
        let history = pane.app.session.history().len();
        for (axis, source) in axes.iter().zip(sources) {
            let shown = crate::props::field_text(
                &pane.row(pattern, SlotId::Origin(*axis)),
                pane.app.session.notation(),
            );
            pane.click(&shown);
            let open = pane.frame(Vec::new());
            assert!(
                open.iter().any(|(run, _)| run == source),
                "the {axis:?} field's edit opens on `{source}`: {open:?}"
            );
            for other in sources.iter().filter(|other| **other != source) {
                assert!(
                    !open.iter().any(|(run, _)| run == other),
                    "and not on `{other}`"
                );
            }
            let away = |pressed| egui::Event::PointerButton {
                pos: Driven::ELSEWHERE,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            pane.frame(vec![away(true), away(false)]);
            pane.quiet();
        }
        assert_eq!(
            pane.app.session.history().len(),
            history,
            "clicking into each field and away wrote nothing"
        );
    }

    /// A session over two instances of one part, each placed into the
    /// world by itself, and the two instance ids in document order.
    ///
    /// The part is never resolved: the hide's admission is a question
    /// about the recipe, so the rows fail to evaluate and the tree draws
    /// them, with their checkboxes, all the same.
    fn two_instances() -> (crate::session::DocSession, [RecipeNodeId; 2]) {
        use pncad::document::{ContentPin, DocRef, DocumentId, Node, Operand, ProfileDoc, VarId};
        let tol = pncad::tolerance::witness();
        let part = DocRef {
            id: DocumentId::derive("shown-part"),
            pin: ContentPin::of_bytes(b"shown-part"),
        };
        let mut doc = ProfileDoc::empty(DocumentId::derive("shown-asm"), tol);
        let insert = |doc: &mut ProfileDoc, node| crate::test_support::insert_into(doc, node, tol);
        let world = || Operand::Var(VarId::WORLD);
        let stand = || pncad::document::stand().bundle();
        let a = insert(&mut doc, Node::instantiate_part(part, world(), stand()));
        let b = insert(&mut doc, Node::instantiate_part(part, world(), stand()));
        let mut session = crate::session::DocSession::inline(doc, tol);
        session.pump();
        (session, [a, b])
    }

    /// **An instance row's tree checkbox is live, silent, and the click
    /// hides its own row's body.**
    #[test]
    fn an_instances_tree_checkbox_is_live_and_hides_its_own_body() {
        let (session, [a, _]) = two_instances();
        let mut pane = Driven::with_seams(Vec::new(), |app| app.session = session);
        let at = pane.each("shown");
        assert_eq!(at.len(), 2, "one checkbox per instance row: {at:?}");
        let quiet = pane.quiet();
        assert!(Driven::gained(&quiet, pane.hovered(at[0])).is_empty());
        pane.click_at(at[0]);
        pane.quiet();
        assert_eq!(
            pane.app.session.display().toggled(),
            &std::collections::BTreeMap::from([(a, false)])
        );
    }

    /// [`two_instances`] with a copy of the first placed into the world,
    /// which reads it as a body: the first instance is one the default
    /// picture leaves out.
    fn an_undrawn_instance() -> (crate::session::DocSession, RecipeNodeId) {
        use pncad::document::{BodyRead, Node, Operand, VarId};
        let (session, [a, _]) = two_instances();
        let tol = session.tol();
        let mut doc = session.committed_doc().clone();
        crate::test_support::insert_into(
            &mut doc,
            Node::place(
                BodyRead::plain(Operand::from(a)),
                Operand::Var(VarId::WORLD),
                pncad::document::stand().bundle(),
            ),
            tol,
        );
        let mut session = crate::session::DocSession::inline(doc, tol);
        session.pump();
        assert!(!session.toggles().body_on(a), "the premise: a reads off");
        (session, a)
    }

    /// **The tree's checkbox on an instance the default leaves out reads
    /// off, so its click shows it.** A checkbox that read the hides
    /// alone would show it on, and its click would ask for it off,
    /// which holds nothing.
    #[test]
    fn the_tree_checkbox_of_an_undrawn_instance_shows_it() {
        let (session, a) = an_undrawn_instance();
        let mut pane = Driven::with_seams(Vec::new(), |app| app.session = session);
        let at = pane.each("shown");
        pane.click_at(at[0]);
        pane.quiet();
        assert_eq!(
            pane.app.session.display().toggled(),
            &std::collections::BTreeMap::from([(a, true)]),
            "the click showed the instance"
        );
    }

    /// **The Properties pane's checkbox reads the same toggle**: over the
    /// selected undrawn instance, a click shows it.
    #[test]
    fn the_properties_checkbox_of_an_undrawn_instance_shows_it() {
        let (session, a) = an_undrawn_instance();
        let mut pane = Driven::with_seams(vec![SessionOp::Select(Selection::Node(a))], |app| {
            app.session = session
        });
        let at = pane.each("shown in viewport");
        assert_eq!(at.len(), 1, "one display toggle: {at:?}");
        pane.click_at(at[0]);
        pane.quiet();
        assert_eq!(
            pane.app.session.display().toggled(),
            &std::collections::BTreeMap::from([(a, true)]),
            "the click showed the instance"
        );
    }

    /// **The startup document on an inline session, landed, with a
    /// cap of its extrude selected** — and, where `stale`, the feature
    /// that made the cap deleted and the run that would drop it
    /// canceled, so the face still resolves against the landed run
    /// while the committed document no longer holds its feature. Drawn
    /// by the whole app; answers the feature the header's Delete names.
    fn a_face_of_the_body(stale: bool) -> (Driven, RecipeNodeId) {
        let tol = pncad::tolerance::witness();
        let (document, _) = crate::scene::plate_with_hole(tol).expect("the startup document");
        let mut session = crate::session::DocSession::inline(document, tol);
        session.pump();
        let face = cap_of(pncad::prelude::CapEnd::End);
        let body = face.feature();
        session.perform(SessionOp::Select(Selection::Face(face)));
        if stale {
            let deleted = session.perform(SessionOp::DeleteNode { node: body });
            assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
            session.perform(SessionOp::CancelEvaluation);
            session.pump();
            assert!(
                session.committed_doc().node(body).is_none(),
                "the committed document dropped the body"
            );
            assert!(
                session
                    .landed_pair()
                    .is_some_and(|(landed, _)| landed.node(body).is_some()),
                "the landed run still holds it"
            );
        }
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.interaction.tooltip_delay = 0.0);
        let mut app =
            ViewerApp::assemble(&ctx, tol).expect("startup that needs no graphics device");
        app.session = session;
        let mut pane = Driven {
            ctx,
            app,
            frame: eframe::Frame::_new_kittest(),
            time: 0.0,
        };
        pane.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        (pane, body)
    }

    /// **A face of a feature the committed document no longer holds
    /// offers a Delete that is drawn, cannot be clicked, and says what
    /// `DeleteNode` would refuse with** — the whole app, the real
    /// pane, the feature dropped through the real door.
    ///
    /// The runtime value that makes it false is a button gated on the
    /// landed run rather than on the document the op reads: hovered,
    /// it would say nothing, and clicked it would push a delete the
    /// edit door then refuses onto the status line.
    ///
    /// **It waits for the pick index, not for [`quiescent`].** The
    /// fixture cancels a run, so the session stays
    /// `Outstanding::Canceled` and the app never reaches "nothing
    /// outstanding". Nothing is running behind the cancel, though, so
    /// once the index attempt for the picture on screen is answered no
    /// worker is left to change the frame.
    #[test]
    fn a_stale_faces_delete_is_disabled_with_the_refusal_it_would_get() {
        let (mut pane, body) = a_face_of_the_body(true);
        let label = pane.app.session.delete_affordance(body).label;
        pane.settled(index_answered, Some(Driven::ELSEWHERE));
        pane.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let quiet = pane.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let hovered = pane.hovered(Driven::only(&quiet, &label));
        let gained = Driven::gained(&quiet, hovered);
        let said = pane
            .app
            .session
            .delete_refusal(body)
            .expect("DeleteNode refuses a node the committed document does not hold")
            .to_string();
        assert_eq!(
            gained,
            vec![said.clone()],
            "the hover is the op's own sentence"
        );
        // Planted: the words a reader gets for this row.
        assert_eq!(
            said,
            format!(
                "the edit was refused: node {} is not live. \
                 Recourse: aim the edit at a node the document holds",
                test_utils::refusal::tag(body.0.digest())
            )
        );
        let before = status(&pane);
        pane.click(&label);
        assert_eq!(status(&pane), before, "a disabled button pushes nothing");
        // The gate and the door, one answer.
        let refused = pane
            .app
            .session
            .perform(SessionOp::DeleteNode { node: body })
            .refusal
            .map(|refusal| refusal.to_string());
        assert_eq!(refused, Some(said), "what the door answers the click");
    }

    /// **The same face on a feature the document holds offers a live
    /// Delete whose hover is its cost and no refusal, and the click
    /// deletes** — the
    /// row that keeps the one above from passing because the harness
    /// missed the button.
    #[test]
    fn a_held_faces_delete_is_live_and_says_only_its_cost() {
        let (mut pane, body) = a_face_of_the_body(false);
        assert!(pane.app.session.delete_refusal(body).is_none());
        let affordance = pane.app.session.delete_affordance(body);
        let label = affordance.label;
        // The cost hover the affordance owes, and no refusal.
        assert_eq!(
            pane.gained_hovering(&label),
            affordance.hover.into_iter().collect::<Vec<_>>()
        );
        pane.click(&label);
        assert!(
            pane.app.session.committed_doc().node(body).is_none(),
            "the click deleted the body"
        );
    }

    /// Every text the app painted once its "Add feature" section is
    /// opened, with the drafts `plant` wrote, in a window tall enough
    /// to hold the section: frames until the app is [`quiescent`],
    /// whose last is where the header is found, one that clicks it,
    /// then frames on a clock one second apart, well past the
    /// section's opening animation.
    fn painted_adding_a_profile(plant: impl FnOnce(&mut crate::drafts::Drafts)) -> Vec<String> {
        let ctx = egui::Context::default();
        let mut app = ViewerApp::assemble(&ctx, pncad::tolerance::witness())
            .expect("startup that needs no graphics device");
        app.drafts.profile_open = true;
        plant(&mut app.drafts);
        let mut frame = eframe::Frame::_new_kittest();
        let mut seconds = 0.0;
        let mut run = |app: &mut ViewerApp, events: Vec<egui::Event>| {
            seconds += 1.0;
            app_frame(
                &ctx,
                app,
                &mut frame,
                egui::vec2(1600.0, 4000.0),
                Some(seconds),
                events,
            )
        };
        let header = settle(&mut app, quiescent, |app| run(app, Vec::new()))
            .into_iter()
            .find(|landed| landed.text == crate::pane::create::ADD_FEATURE)
            .expect("the creation section's header is painted")
            .allocated
            .center();
        let button = |pressed| egui::Event::PointerButton {
            pos: header,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        run(
            &mut app,
            vec![
                egui::Event::PointerMoved(header),
                button(true),
                button(false),
            ],
        );
        run(&mut app, Vec::new());
        run(&mut app, Vec::new())
            .into_iter()
            .map(|landed| landed.text)
            .collect()
    }

    /// Which of three withholding reasons the app paints — the face prompt,
    /// the empty chain's, the bore's refusal — drawing `on_face` with no
    /// face held, `shape` chosen, a bore planted as wide as the radius and
    /// the chain planted empty.
    fn withheld_reason_said(on_face: bool, shape: crate::forms::ShapeKind) -> Vec<&'static str> {
        const SAID: [&str; 3] = [
            crate::pane::create::PICK_A_FACE,
            "add a step to the chain",
            "the bore must be smaller than the radius",
        ];
        let painted = painted_adding_a_profile(|drafts| {
            drafts.sketch_on_face = on_face;
            drafts.profile_shape = Some(shape);
            drafts.profile_bored = true;
            drafts.profile_bore = drafts.profile_radius;
            drafts.profile_path.clear();
        });
        assert!(
            painted
                .iter()
                .any(|text| text == crate::pane::create::ADD_PROFILE),
            "the add-profile form is drawn in this frame: {painted:?}"
        );
        SAID.into_iter()
            .filter(|said| painted.iter().any(|text| text.starts_with(said)))
            .collect()
    }

    /// **The add-profile form says one withholding reason, and which one is
    /// decided once**: drawing on a face with none held and an empty
    /// chain, the face prompt, which comes first in the form; with an
    /// over-wide bore, the refusal, on a face or not.
    #[test]
    fn the_add_profile_form_says_a_refusal_before_a_missing_input_and_the_face_first() {
        use crate::forms::ShapeKind;
        assert_eq!(
            withheld_reason_said(true, ShapeKind::Path),
            [crate::pane::create::PICK_A_FACE]
        );
        assert_eq!(
            withheld_reason_said(true, ShapeKind::Circle),
            ["the bore must be smaller than the radius"]
        );
        assert_eq!(
            withheld_reason_said(false, ShapeKind::Circle),
            ["the bore must be smaller than the radius"]
        );
        // The empty chain is said off the face, so the first case above
        // held it back rather than never drawing it.
        assert_eq!(
            withheld_reason_said(false, ShapeKind::Path),
            ["add a step to the chain"]
        );
    }

    /// **Every tool the chrome has opens from it**: for each kind, with
    /// the Properties section that hosts the kind's activation button
    /// clicked open, the app paints that button once, in
    /// `ToolKind::button`'s words, and clicking it opens that kind.
    ///
    /// Which section hosts a kind is [`section_of`]'s exhaustive match,
    /// so a new kind does not compile until it is placed, and the sweep
    /// over `ToolKind::ALL` then clicks it. A kind placed in a section
    /// must not be painted before that section opens, so a button moved
    /// out of its section reddens the row rather than passing. One app
    /// serves the sweep: each kind's tool is closed and its section
    /// collapsed again before the next, so every kind is found on the
    /// startup layout.
    #[test]
    fn every_tool_opens_from_its_activation_button() {
        use crate::tools::ToolKind;
        let mut driven = Driven::with(Vec::new());
        let opened: Vec<Option<ToolKind>> = ToolKind::ALL
            .into_iter()
            .map(|kind| {
                let section = section_of(kind);
                assert_eq!(driven.app.tools.open_kind(), None, "before {kind:?}");
                let closed = driven.frame(Vec::new());
                assert!(
                    !closed.iter().any(|(run, _)| *run == kind.button()),
                    "{kind:?}'s button is painted before {section:?} opens: {closed:?}"
                );
                driven.click(section);
                driven.click(&kind.button());
                let opened = driven.app.tools.open_kind();
                driven.app.tools.close();
                driven.click(section);
                opened
            })
            .collect();
        assert_eq!(
            opened,
            ToolKind::ALL.map(Some),
            "what each kind's button opened, one entry per kind visited"
        );
    }

    /// **The creation doors that are not tools are reachable too**: the
    /// extrude form is drawn while a profile is selected and not while
    /// anything else is (its door's own kind test) — with every
    /// creation section still collapsed, so a profile selected finds it
    /// without opening a header — and the `Add part…` button, under
    /// its section, opens the part chooser. The add-profile form is the
    /// subject of rows of its own, which open its door and read it.
    #[test]
    fn the_extrude_form_and_the_part_chooser_are_reachable() {
        use crate::pane::create::{ADD_PART, ADD_PROFILE_DOOR, ASSEMBLE_PARTS, EXTRUDE};
        // The button's words for `node`, as the form spells them.
        let button = |driven: &Driven, node: RecipeNodeId| {
            format!(
                "{EXTRUDE} {}",
                pncad::document::Speaker::of(driven.app.session.doc()).node(node)
            )
        };
        let offers = |painted: &[(String, egui::Rect)], words: &str| {
            painted.iter().any(|(run, _)| run == words)
        };
        let mut driven = Driven::with(vec![SessionOp::Select(Selection::Node(extrude()))]);
        let painted = driven.frame(Vec::new());
        let refused = button(&driven, extrude());
        assert!(
            !offers(&painted, &refused) && !offers(&painted, EXTRUDE),
            "an extrude selected is not a profile, so no extrude is offered: {painted:?}"
        );
        driven.select(Selection::Node(startup_node(0)));
        let painted = driven.frame(Vec::new());
        let offered = button(&driven, startup_node(0));
        assert!(
            !offers(&painted, ADD_PROFILE_DOOR),
            "the creation sections are collapsed in this frame: {painted:?}"
        );
        assert!(
            offers(&painted, &offered),
            "the startup profile selected offers {offered:?} with the sections collapsed: \
             {painted:?}"
        );
        assert!(driven.app.part_chooser.is_none(), "the startup app");
        driven.click(ASSEMBLE_PARTS);
        driven.click(ADD_PART);
        assert!(
            driven.app.part_chooser.is_some(),
            "{ADD_PART:?} opens the chooser"
        );
    }

    /// **The Properties pane at rest is its doors**: with nothing
    /// selected and nothing open, no form's fields or commit is
    /// painted — the add-profile form behind `Add profile…` even with
    /// its section open, the add-variable form behind `New variable…` —
    /// and each door opens its form. The add-variable form, open with
    /// no dimension picked, says why its commit waits in the painted
    /// frame rather than only on a hover.
    #[test]
    fn the_properties_pane_at_rest_draws_doors_and_each_opens_its_form() {
        use crate::pane::create::{ADD_FEATURE, ADD_PROFILE, ADD_PROFILE_DOOR};
        use crate::pane::properties::{NEW_VARIABLE, NEW_VARIABLE_COMMIT};
        let offers = |painted: &[(String, egui::Rect)], words: &str| {
            painted.iter().any(|(run, _)| run == words)
        };
        let mut driven = Driven::with(Vec::new());
        let rest = driven.frame(Vec::new());
        for (door, form) in [
            (NEW_VARIABLE, NEW_VARIABLE_COMMIT),
            (ADD_FEATURE, ADD_PROFILE),
        ] {
            assert!(
                offers(&rest, door) && !offers(&rest, form),
                "at rest {door:?} is painted and {form:?} is not: {rest:?}"
            );
        }
        driven.click(ADD_FEATURE);
        let section = driven.frame(Vec::new());
        assert!(
            offers(&section, ADD_PROFILE_DOOR) && !offers(&section, ADD_PROFILE),
            "the open section offers the form's door, not the form: {section:?}"
        );
        driven.click(ADD_PROFILE_DOOR);
        assert!(
            offers(&driven.frame(Vec::new()), ADD_PROFILE),
            "the door opens the add-profile form"
        );
        driven.click(NEW_VARIABLE);
        driven.app.drafts.new_variable_name = "gap".to_owned();
        let open = driven.frame(Vec::new());
        assert!(
            offers(&open, NEW_VARIABLE_COMMIT) && offers(&open, "pick a dimension first"),
            "the open form paints its commit and why it waits: {open:?}"
        );
    }

    /// **The variables list paints each variable's value**: a length
    /// declared in millimetres is listed as `t = 20 mm`, the line
    /// [`crate::pane::properties::variable_line`] spells.
    #[test]
    fn the_variables_list_paints_each_variables_value() {
        let driven = &mut Driven::with(vec![SessionOp::DeclareVar {
            name: pncad::document::VarName::from_static("t"),
            value: crate::props::doc_variable(
                pncad::document::Dimension::Length,
                crate::props::SlotValue::Continuous(0.02),
                Some(pncad::quantity::MM.def()),
            ),
        }]);
        let painted = driven.frame(Vec::new());
        assert!(
            painted.iter().any(|(run, _)| run == "t = 20 mm"),
            "the list says t's value: {painted:?}"
        );
    }

    /// **A definition that reads nothing says why under its list
    /// line** (review of #6242): `w = t / scalar(n - 2)` at `n = 2`
    /// reads `?` in the list, and the fault is painted, so a list that dropped the note
    /// or showed a stale value goes red.
    #[test]
    fn the_variables_list_says_why_a_definition_reads_nothing() {
        let length = |value| {
            crate::props::doc_variable(
                pncad::document::Dimension::Length,
                crate::props::SlotValue::Continuous(value),
                Some(pncad::quantity::MM.def()),
            )
        };
        let mut driven = Driven::with(vec![
            SessionOp::DeclareVar {
                name: pncad::document::VarName::from_static("t"),
                value: length(0.02),
            },
            SessionOp::DeclareVar {
                name: pncad::document::VarName::from_static("n"),
                value: crate::props::doc_variable(
                    pncad::document::Dimension::Count,
                    crate::props::SlotValue::Count(3),
                    None,
                ),
            },
            SessionOp::DeclareVar {
                name: pncad::document::VarName::from_static("w"),
                value: length(0.04),
            },
        ]);
        let w = crate::props::variable_rows(driven.app.session.doc())
            .into_iter()
            .find(|row| row.label.to_string() == "w")
            .expect("w is declared")
            .var;
        let n = crate::props::variable_rows(driven.app.session.doc())
            .into_iter()
            .find(|row| row.label.to_string() == "n")
            .expect("n is declared")
            .var;
        let defined = driven.app.session.perform(SessionOp::SetVariableText {
            var: w,
            text: "t / scalar(n - 2)".to_owned(),
        });
        assert!(defined.refusal.is_none(), "w is defined: {defined:?}");
        let moved = driven.app.session.perform(SessionOp::SetVariable {
            var: n,
            value: crate::props::SlotValue::Count(2),
        });
        assert!(moved.refusal.is_none(), "n moves to 2: {moved:?}");
        let row = crate::props::defined_rows(driven.app.session.doc())
            .into_iter()
            .find(|row| row.var == w)
            .expect("w is defined");
        let fault = row
            .value
            .as_ref()
            .expect_err("the definition reads nothing");
        let painted = driven.frame(Vec::new());
        assert!(
            painted
                .iter()
                .any(|(run, _)| run == &format!("w = {} = {}", row.formula, crate::props::NO_VALUE)),
            "the list says w reads nothing: {painted:?}"
        );
        assert!(
            painted.iter().any(|(run, _)| run == &format!("w: {fault}")),
            "and why, under its line: {painted:?}"
        );
        // Selected, its own row says why, and the pane says it once.
        driven.select(Selection::Variable(w));
        let painted = driven.frame(Vec::new());
        let said = painted
            .iter()
            .filter(|(run, _)| run == &format!("w: {fault}"))
            .count();
        assert_eq!(said, 1, "the reason is said once: {painted:?}");
    }

    /// **The add-variable form's commit is live exactly when a click
    /// declares**: a `Count` over a field holding `NaN` is refused by
    /// `SlotValue::of`, so the commit is disabled and the refusal is
    /// painted under the row — a click pushes nothing and the form
    /// stays open.
    #[test]
    fn the_add_variable_commit_waits_on_a_value_its_dimension_refuses() {
        use crate::pane::properties::{NEW_VARIABLE, NEW_VARIABLE_COMMIT};
        use pncad::document::Dimension;
        let mut driven = Driven::with(Vec::new());
        driven.click(NEW_VARIABLE);
        driven.app.drafts.new_variable_name = "n".to_owned();
        driven.app.drafts.new_variable_dimension = Some(Dimension::Count);
        driven.app.drafts.new_variable_value = f64::NAN;
        let refusal = crate::props::SlotValue::of(Dimension::Count, f64::NAN)
            .expect_err("a NaN count is refused")
            .to_string();
        let painted = driven.frame(Vec::new());
        assert!(
            painted.iter().any(|(run, _)| *run == refusal),
            "the refusal is painted under the row: {painted:?}"
        );
        let vars = driven.app.session.committed_doc().var_named("n");
        driven.click(NEW_VARIABLE_COMMIT);
        driven.quiet();
        assert_eq!(
            driven.app.session.committed_doc().var_named("n"),
            vars,
            "nothing was declared"
        );
        assert!(driven.app.drafts.new_variable_open, "the form stays open");
    }

    /// **The add-variable form closes when its declaration is accepted,
    /// and a refused one keeps what was typed**: with a drag in flight
    /// the declare is refused (`Refusal::GestureInFlight`), so the form
    /// is still open on the typed name; once the drag is cancelled the
    /// same click declares and the form closes.
    #[test]
    fn the_add_variable_form_closes_on_acceptance_and_keeps_a_refused_draft() {
        use crate::pane::properties::{NEW_VARIABLE, NEW_VARIABLE_COMMIT};
        use pncad::document::Dimension;
        let mut driven = Driven::with(Vec::new());
        driven.click(NEW_VARIABLE);
        driven.app.drafts.new_variable_name = "gap".to_owned();
        driven.app.drafts.new_variable_dimension = Some(Dimension::Length);
        driven.app.drafts.new_variable_value = 0.002;
        driven.app.perform_batch(vec![SessionOp::BeginGesture {
            node: extrude(),
            slot: pncad::document::SlotId::Distance,
        }]);
        driven.click(NEW_VARIABLE_COMMIT);
        assert!(
            driven
                .app
                .session
                .committed_doc()
                .var_named("gap")
                .is_none(),
            "refused while the drag is in flight"
        );
        assert!(
            driven.app.drafts.new_variable_open && driven.app.drafts.new_variable_name == "gap",
            "a refused declaration keeps the form and what was typed"
        );
        driven.app.perform_batch(vec![SessionOp::CancelGesture]);
        driven.click(NEW_VARIABLE_COMMIT);
        assert!(
            driven
                .app
                .session
                .committed_doc()
                .var_named("gap")
                .is_some(),
            "declared"
        );
        assert!(
            !driven.app.drafts.new_variable_open,
            "the form closed with its accepted declaration"
        );
    }

    /// **A fresh directory under the temp dir holding one stored part**
    /// (`test_support::boss_on_block`), for a row that authors an
    /// assembly beside it: the directory, the part, and the file it was
    /// stored in. `name` names the directory, made unique by the process.
    fn part_workspace(
        name: &str,
        tol: pncad::geom_core::Tol,
    ) -> (
        std::path::PathBuf,
        pncad::document::Doc<pncad::document::ProfileProgram>,
        std::path::PathBuf,
    ) {
        let dir = std::env::temp_dir().join(format!("{name} {}", std::process::id()));
        if dir.exists() {
            std::fs::remove_dir_all(&dir).expect("clear the fixture directory");
        }
        std::fs::create_dir_all(&dir).expect("create the fixture directory");
        let (part, _, _) = crate::test_support::boss_on_block(&format!("{name}-part"), tol);
        let file = pncad::workspace::Workspace::open(&dir)
            .expect("the empty workspace opens")
            .create(&part, tol)
            .expect("the part stores");
        (dir, part, file)
    }

    /// **The instance the part chooser inserted** into `doc`.
    fn the_instance(doc: &pncad::document::Doc<pncad::document::ProfileProgram>) -> RecipeNodeId {
        doc.ids()
            .iter()
            .copied()
            .find(|id| {
                matches!(
                    doc.node(*id),
                    Some(pncad::document::Node::InstantiatePart { .. })
                )
            })
            .expect("the instance went in")
    }

    /// **The free-move cancel door is on the toolbar exactly while a
    /// probe is in flight**: absent before it begins and after it is
    /// cancelled, present between — and the drag's door stays off, its
    /// own gesture not being in flight.
    #[test]
    fn the_free_move_cancel_door_is_on_the_toolbar_only_while_a_probe_is_in_flight() {
        const DRAG: &str = "Cancel drag";
        const FREE_MOVE: &str = "Cancel free-move";
        let shown = |painted: &[(String, egui::Rect)], label: &str| {
            painted.iter().any(|(text, _)| text == label)
        };
        let tol = pncad::tolerance::witness();
        let (dir, part, _) = part_workspace("free-move door", tol);
        let mut driven = Driven::with(vec![
            SessionOp::NewDocument {
                name: "free-move-door".to_owned(),
            },
            SessionOp::Save(dir.join("assembly.pncad")),
            SessionOp::AddInstance { id: part.id() },
        ]);
        driven.settle();
        let instance = the_instance(driven.app.session.committed_doc());
        // A space of its own, which the probe moves.
        driven.app.perform_batch(vec![SessionOp::IntoOwnSpace {
            placement: instance,
        }]);
        driven.settle();
        let rest = driven.frame(Vec::new());
        assert!(!shown(&rest, FREE_MOVE), "no probe, no door: {rest:?}");
        driven
            .app
            .perform_batch(vec![SessionOp::BeginFreeMove { node: instance }]);
        assert!(
            driven.app.session.display().probing().is_some(),
            "the probe began: {:?}",
            status(&driven)
        );
        let held = driven.frame(Vec::new());
        assert!(
            shown(&held, FREE_MOVE) && !shown(&held, DRAG),
            "a probe in flight puts its own door on the toolbar and no other: {held:?}"
        );
        driven.click(FREE_MOVE);
        let after = driven.frame(Vec::new());
        assert!(
            !shown(&after, FREE_MOVE),
            "the door cancelled the probe and went with it: {after:?}"
        );
        std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
    }

    /// **An undo's withdrawal reaches the status line** (DI5): a
    /// free-move frame is keyed by its space, and an undo past the edit
    /// that gave the copy a space of its own takes the space, so the
    /// edit's own prune discards the frame and the batch says it. Drawn
    /// through the real app, frame by frame.
    #[test]
    fn an_undos_withdrawal_reaches_the_status_line() {
        let tol = pncad::tolerance::witness();
        let (dir, part, _) = part_workspace("undo notice", tol);
        let mut driven = Driven::with(vec![
            SessionOp::NewDocument {
                name: "undo-notice".to_owned(),
            },
            SessionOp::Save(dir.join("assembly.pncad")),
            SessionOp::AddInstance { id: part.id() },
        ]);
        driven.settle();
        let instance = the_instance(driven.app.session.committed_doc());
        // A space of its own, which the probe moves.
        driven.app.perform_batch(vec![SessionOp::IntoOwnSpace {
            placement: instance,
        }]);
        driven.settle();
        let space = driven
            .app
            .session
            .committed_doc()
            .node_space(instance)
            .expect("the copy is placed into a space");
        driven.app.perform_batch(vec![
            SessionOp::BeginFreeMove { node: instance },
            SessionOp::PreviewFreeMove {
                node: instance,
                frame: pncad::document::Frame::translation([0.0, 0.0, 0.05]),
            },
            SessionOp::CommitFreeMove { node: instance },
        ]);
        assert!(
            driven.app.session.display().free_move_of(space).is_some(),
            "the probe committed: {:?}",
            status(&driven)
        );

        // The undo takes the space, and its own prune the frame.
        driven.app.perform_batch(vec![SessionOp::Undo]);
        assert!(
            driven.app.session.display().free_move_of(space).is_none(),
            "the undo's prune dropped the frame"
        );
        let said = status(&driven).unwrap_or_default();
        assert!(
            said.contains("which is not in the document, so that free-move is gone"),
            "the undo's withdrawal is on the status line: {said:?}"
        );
        driven.settle();
        std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
    }

    /// **The free-move probe drives the instance its panel row draws.**
    /// Of two instances, each in a space of its own, the selected one's
    /// probe is dragged through the whole app: the probe held mid-drag
    /// is the selected instance's, and on release the selected
    /// instance's space carries the free-move and the other's does not.
    /// `widgets::free_move_gesture` minting its vocabulary for any other
    /// node reds here.
    #[test]
    fn the_free_move_probe_drives_the_instance_its_row_draws() {
        let tol = pncad::tolerance::witness();
        let (dir, part, _) = part_workspace("probe target", tol);
        let mut driven = Driven::with(vec![
            SessionOp::NewDocument {
                name: "probe-target".to_owned(),
            },
            SessionOp::Save(dir.join("assembly.pncad")),
            SessionOp::AddInstance { id: part.id() },
            SessionOp::AddInstance { id: part.id() },
        ]);
        driven.settle();
        let doc = driven.app.session.committed_doc();
        let instances: Vec<RecipeNodeId> = doc
            .ids()
            .iter()
            .copied()
            .filter(|id| matches!(doc.node(*id), Some(Node::InstantiatePart { .. })))
            .collect();
        let [selected, other] = instances[..] else {
            panic!("the fixture holds two instances: {instances:?}");
        };
        driven.app.perform_batch(vec![
            SessionOp::IntoOwnSpace {
                placement: selected,
            },
            SessionOp::IntoOwnSpace { placement: other },
        ]);
        driven.settle();
        let space_of = |driven: &Driven, node| {
            driven
                .app
                .session
                .committed_doc()
                .node_space(node)
                .expect("each copy is placed into a space of its own")
        };
        let (selected_space, other_space) = (space_of(&driven, selected), space_of(&driven, other));
        assert_ne!(selected_space, other_space, "two spaces");
        driven.select(Selection::Node(selected));
        let painted = driven.quiet();
        let label = painted
            .iter()
            .find(|(run, _)| run.starts_with("free-move probe ("))
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("the selected copy is offered the probe: {painted:?}"));
        // The probe's first box: the first number painted below its
        // label.
        let at = painted
            .iter()
            .filter(|(run, rect)| rect.top() >= label.bottom() && run.parse::<f64>().is_ok())
            .map(|(_, rect)| rect.center())
            .min_by(|a, b| a.y.total_cmp(&b.y).then(a.x.total_cmp(&b.x)))
            .unwrap_or_else(|| panic!("the probe's boxes are painted: {painted:?}"));

        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        driven.frame(vec![egui::Event::PointerMoved(at)]);
        driven.frame(vec![button(at, true)]);
        let mut to = at;
        for _ in 0..4 {
            to += egui::vec2(12.0, 0.0);
            driven.frame(vec![egui::Event::PointerMoved(to)]);
        }
        assert_eq!(
            driven.app.session.display().probing(),
            Some(selected),
            "mid-drag, the probe held is the selected copy's: {:?}",
            status(&driven)
        );
        driven.frame(vec![button(to, false)]);
        driven.frame(Vec::new());
        let display = driven.app.session.display();
        let removed = std::fs::remove_dir_all(&dir);
        assert_eq!(display.probing(), None, "the release committed the probe");
        assert!(
            display.free_move_of(selected_space).is_some(),
            "the selected copy's space carries the free-move: {:?}",
            status(&driven)
        );
        assert!(
            display.free_move_of(other_space).is_none(),
            "the other copy's space does not"
        );
        removed.expect("the fixture directory is removable");
    }

    /// **The Properties pane reads the evaluation the op does**, so
    /// between an edit and its landing it says what the probe would
    /// refuse — the copy's picture is not known yet — rather than offer
    /// the control off the older landing. The copy is relabelled by an
    /// edit whose run the [`Gated`] seam holds.
    #[test]
    fn the_pane_does_not_offer_the_probe_off_an_older_landing() {
        let tol = pncad::tolerance::witness();
        let (dir, part, _) = part_workspace("pane older landing", tol);
        let (mut driven, open) = gated();
        driven.app.perform_batch(vec![
            SessionOp::NewDocument {
                name: "pane-older-landing".to_owned(),
            },
            SessionOp::Save(dir.join("assembly.pncad")),
            SessionOp::AddInstance { id: part.id() },
        ]);
        driven.settle();
        let instance = the_instance(driven.app.session.committed_doc());
        driven.perform(SessionOp::IntoOwnSpace {
            placement: instance,
        });
        driven.select(Selection::Node(instance));
        let painted = driven.quiet();
        assert!(
            painted
                .iter()
                .any(|(run, _)| run.starts_with("free-move probe (")),
            "the copy in a space of its own is offered the probe: {painted:?}"
        );

        // An edit, its run held: no landing answers the document.
        open.store(false, std::sync::atomic::Ordering::SeqCst);
        driven.app.perform_batch(vec![
            SessionOp::SetLabel {
                node: instance,
                label: Some(pncad::document::Label::new("held").expect("a label")),
            },
            SessionOp::Select(Selection::Node(instance)),
        ]);
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let painted = driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert!(
            driven.app.session.space_evaluation().is_none(),
            "the held run leaves no landing that answers the document"
        );
        let text: String = painted
            .iter()
            .map(|(run, _)| run.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            text.contains("has no picture yet"),
            "the pane says the picture is not known yet: {text}"
        );
        assert!(
            !text.contains("free-move probe ("),
            "and offers no probe control: {text}"
        );
        open.store(true, std::sync::atomic::Ordering::SeqCst);
        std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
    }

    /// The product the committed document's slot reads, piece by piece.
    fn product_of(driven: &Driven) -> Vec<pncad::document::VarId> {
        crate::world::product_reads(driven.app.session.committed_doc())
    }

    /// **"Add to the product" is offered on the body under a viewport
    /// pick, and the product's recourse is the same door.** A pick on
    /// the startup plate's cap is on its world copy, which the product
    /// reads: the pane says so and offers to take it out, where the
    /// minting extrude would have offered nothing. Taken out, the slot
    /// reads nothing: the pane offers the door on the same pick, and the
    /// `EmptyProduct` recourse offers it on the drawn copy. Each click
    /// is one action, which one undo takes back.
    #[test]
    fn the_product_door_is_offered_on_a_pick_and_as_the_empty_products_recourse() {
        use crate::pane::properties::{ADD_TO_THE_PRODUCT, TAKE_OUT_OF_THE_PRODUCT};
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        let copy = driven
            .app
            .session
            .committed_doc()
            .output(placement(), 0)
            .expect("the startup placement's copy");
        assert_eq!(
            product_of(&driven),
            [copy],
            "the startup product is its copy"
        );
        driven.select(Selection::Face(cap_of(pncad::prelude::CapEnd::End)));
        let painted = driven.quiet();
        assert!(
            painted.iter().any(|(run, _)| run == "in the product")
                && !painted.iter().any(|(run, _)| run == ADD_TO_THE_PRODUCT),
            "the pick is on the copy the product reads: {painted:?}"
        );
        driven.click(TAKE_OUT_OF_THE_PRODUCT);
        driven.quiet();
        assert!(
            product_of(&driven).is_empty(),
            "the copy is out of the product"
        );

        // The same pick now offers the door; one click is one action.
        let history = driven.app.session.history().len();
        driven.click(ADD_TO_THE_PRODUCT);
        driven.quiet();
        assert_eq!(product_of(&driven), [copy], "the pick's copy is added");
        assert_eq!(
            driven.app.session.history().len(),
            history + 1,
            "one action"
        );
        driven.perform(SessionOp::Undo);
        assert!(product_of(&driven).is_empty(), "one undo takes it back");

        // The empty product's recourse: the door, on the drawn copy.
        driven.perform(SessionOp::Select(Selection::None));
        let recourse = crate::frame::product_recourse(
            driven.app.session.product_fault(),
            driven.app.session.committed_doc(),
        );
        let [(label, SessionOp::AddToProduct { node, body: None })] = &recourse[..] else {
            panic!("the recourse offers the door on the one drawn copy: {recourse:?}")
        };
        assert_eq!(*node, placement());
        driven.click(label);
        driven.quiet();
        assert_eq!(
            product_of(&driven),
            [copy],
            "the recourse's door adds the copy"
        );
        assert!(
            crate::frame::product_recourse(
                driven.app.session.product_fault(),
                driven.app.session.committed_doc(),
            )
            .is_empty(),
            "a product that gathers names no recourse"
        );
    }

    /// **A copy the slot reads from outside the world is "in the
    /// product", and its recourse moves it back.** The startup plate's
    /// copy, given a space of its own, is still what the slot reads: a
    /// pick on it says so and offers no second add. The fault's recourse
    /// is the door on that copy, which puts it back into the world.
    #[test]
    fn a_stray_copy_reads_in_the_product_and_its_recourse_moves_it_back() {
        use crate::pane::properties::ADD_TO_THE_PRODUCT;
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        driven.perform(SessionOp::IntoOwnSpace {
            placement: placement(),
        });
        driven.select(Selection::Face(cap_of(pncad::prelude::CapEnd::End)));
        let painted = driven.quiet();
        assert!(
            painted.iter().any(|(run, _)| run == "in the product")
                && !painted.iter().any(|(run, _)| run == ADD_TO_THE_PRODUCT),
            "the stray copy is in the product, not offered again: {painted:?}"
        );
        let recourse = crate::frame::product_recourse(
            driven.app.session.product_fault(),
            driven.app.session.committed_doc(),
        );
        let [(label, _)] = &recourse[..] else {
            panic!("the fault offers one action: {recourse:?}")
        };
        driven.click(label);
        driven.quiet();
        assert_eq!(
            driven.app.session.committed_doc().node_space(placement()),
            Some(pncad::document::VarId::WORLD),
            "the recourse moved the copy back into the world"
        );
        assert!(
            driven.app.session.product_fault().is_none(),
            "{:?}",
            driven.app.session.product_fault()
        );
    }

    /// **"Add to the product" is offered on a drawn body's tree row, and
    /// as the `unused` finding's recourse.** A fresh revolve lands
    /// unplaced and nothing reads it, so the checks report it unused
    /// with the door as its action; selecting its row offers the door
    /// in the pane. Either click places it into the world by the
    /// identity bundle and adds that copy, as one action.
    #[test]
    fn the_product_door_is_offered_on_a_tree_row_and_as_the_unused_findings_recourse() {
        use crate::pane::properties::ADD_TO_THE_PRODUCT;
        let (mut driven, revolve) = with_revolve(|_| vec![SessionOp::Select(Selection::None)]);
        let before = product_of(&driven);
        // The unused finding's row, in the Checks window.
        driven.app.checks_shown = true;
        let rows = crate::frame::check_rows(
            driven.app.session.checks().expect("the landing checked"),
            driven.app.session.landed_pair().expect("landed").0,
        );
        assert!(
            rows.iter().any(|row| row.add_to_product == Some(revolve)),
            "the revolve's unused finding offers the door"
        );
        let painted = driven.quiet();
        assert_eq!(
            painted
                .iter()
                .filter(|(run, _)| run == ADD_TO_THE_PRODUCT)
                .count(),
            1,
            "one door, on the one unused body: {painted:?}"
        );
        driven.click(ADD_TO_THE_PRODUCT);
        driven.quiet();
        let doc = driven.app.session.committed_doc();
        let [copy] = crate::world::placements_of(doc, revolve)[..] else {
            panic!("the door places the revolve into the world once")
        };
        assert_eq!(doc.node_space(copy), Some(pncad::document::VarId::WORLD));
        let added = doc.outputs(copy);
        assert_eq!(
            product_of(&driven),
            before.iter().copied().chain(added).collect::<Vec<_>>(),
            "the copy joins what the product read"
        );
        driven.perform(SessionOp::Undo);
        assert_eq!(product_of(&driven), before, "one undo takes it back");

        // The tree row: selecting the revolve offers the door in the pane.
        driven.app.checks_shown = false;
        driven.select(Selection::Node(revolve));
        driven.click(ADD_TO_THE_PRODUCT);
        driven.quiet();
        assert_eq!(
            crate::world::placements_of(driven.app.session.committed_doc(), revolve).len(),
            1,
            "the row's door placed the revolve"
        );
        assert_eq!(product_of(&driven).len(), before.len() + 1);
    }

    /// **An instance from the part chooser takes the label its field
    /// holds**: the chooser proposes `InstantiatePart N` above its
    /// listing, a text typed over the proposal is kept in the field, and
    /// a pick inserts the instance with that label, spending the draft.
    #[test]
    fn the_part_chooser_labels_the_instance_it_inserts() {
        use crate::pane::create::{ADD_PART, INSTANCE_NOUN};
        let tol = pncad::tolerance::witness();
        let (dir, _, part_file) = part_workspace("part chooser label", tol);
        let part_name = part_file
            .file_name()
            .expect("a stored part has a file name")
            .to_string_lossy()
            .into_owned();
        let mut driven = Driven::with(vec![SessionOp::Save(dir.join("assembly.pncad"))]);
        driven.settle();
        driven.click(crate::pane::create::ASSEMBLE_PARTS);
        driven.click(ADD_PART);

        let painted = driven.frame(Vec::new());
        let proposal = Driven::only(&painted, &format!("{INSTANCE_NOUN} 1"));
        let label_at = Driven::only(&painted, "label");
        let part_at = Driven::only(&painted, &part_name);
        assert!(
            label_at.y < part_at.y,
            "the label row stands above the listing: {painted:?}"
        );

        // Typed over the proposal: focus the field, select it all,
        // type.
        let press = |pressed| egui::Event::PointerButton {
            pos: proposal,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        driven.frame(vec![egui::Event::PointerMoved(proposal)]);
        driven.frame(vec![press(true), press(false)]);
        driven.frame(vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        }]);
        driven.frame(vec![egui::Event::Text("left post".to_owned())]);
        assert_eq!(
            driven
                .app
                .drafts
                .creation_labels
                .get(INSTANCE_NOUN)
                .map(String::as_str),
            Some("left post"),
            "the field keeps what was typed"
        );

        // The part's own `add`, on the row its file name is on.
        let painted = driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let part_at = Driven::only(&painted, &part_name);
        let add = painted
            .iter()
            .filter(|(run, rect)| run == "add" && (rect.center().y - part_at.y).abs() < 1.0)
            .map(|(_, rect)| rect.center())
            .next()
            .unwrap_or_else(|| panic!("the part's row has an add button: {painted:?}"));
        let press = |pressed| egui::Event::PointerButton {
            pos: add,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        driven.frame(vec![egui::Event::PointerMoved(add)]);
        driven.frame(vec![press(true), press(false)]);
        driven.quiet();

        let doc = driven.app.session.doc();
        let instances: Vec<_> = doc
            .ids()
            .iter()
            .filter(|id| {
                doc.node(**id)
                    .is_some_and(|node| pncad::document::node_kind_noun(node) == INSTANCE_NOUN)
            })
            .map(|id| doc.label(*id).map(|label| label.as_str().to_owned()))
            .collect();
        assert_eq!(
            instances,
            [Some("left post".to_owned())],
            "one instance, labelled as typed"
        );
        assert!(
            driven.app.part_chooser.is_none(),
            "the pick closes the chooser"
        );
        assert_eq!(
            driven.app.drafts.creation_labels.get(INSTANCE_NOUN),
            None,
            "the landed instance spends the draft"
        );
        std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
    }

    /// A cap of the startup extrude, as a face pick on its drawn copy:
    /// the extrude's cap, which the copy carries as it is, on the
    /// placement.
    fn cap_of(end: pncad::prelude::CapEnd) -> crate::session::FaceSelection {
        crate::session::FaceSelection {
            name: pncad::prelude::StableName {
                kind: pncad::prelude::EntityKind::Face,
                node: extrude(),
                path: vec![pncad::prelude::RoleSeg::Cap(end)],
            },
            node: placement(),
            body: 0,
        }
    }

    /// **`selections` as the viewport's clicks deliver them**: fed to
    /// the open tool (`Tools::feed`, none declined), then performed as
    /// the frame's batch.
    fn clicked(app: &mut ViewerApp, selections: Vec<Selection>) {
        let ops: Vec<SessionOp> = selections.into_iter().map(SessionOp::Select).collect();
        let declined = app.tools.feed(
            app.session.doc(),
            app.picks.index_or_why(app.fit.as_ref()),
            &ops,
        );
        assert!(declined.is_empty(), "{declined:?}");
        app.perform_batch(ops);
    }

    impl Driven {
        /// The index the picture on screen was drawn from — the one
        /// the viewport marks against (`pane::viewport::drawn_index`).
        fn on_screen(&self) -> &crate::pickindex::PickIndex {
            crate::pane::viewport::drawn_index(self.app.picks.index(), self.app.scene_key.clone())
                .expect("the settled app draws from an index")
        }

        /// **What the viewport marks, over this app's own state**:
        /// `pane::viewport::frame_marks`, the one door the frame draws
        /// its marks from, handed what the frame hands it.
        fn marks(&self) -> crate::pane::viewport::Composed {
            crate::pane::viewport::frame_marks(
                Some(self.on_screen()),
                &self.app.session.display_view(),
                &self.app.session,
                &self.app.tools,
                &self.app.drafts,
                self.app.profile_drawn.create,
            )
        }

        /// The held faces' ids the viewport marks.
        fn held(&self) -> [u32; crate::marks::HELD_FACES] {
            self.marks().highlight().held
        }

        /// The patch `face` names, which a mark of it must light.
        fn patch_of(&self, face: &crate::session::FaceSelection) -> u32 {
            let ids = self.on_screen().ids_of_target(face);
            assert_eq!(ids.len(), 1, "{face:?} is drawn once");
            ids[0]
        }

        /// Deliver `selections` as clicks ([`clicked`]), then settle.
        fn click_through(&mut self, selections: Vec<Selection>) {
            clicked(&mut self.app, selections);
            self.quiet();
        }

        /// One click's selection, delivered and settled.
        fn select(&mut self, selection: Selection) {
            self.click_through(vec![selection]);
        }

        /// Perform `op`, then settle.
        fn perform(&mut self, op: SessionOp) {
            self.app.perform_batch(vec![op]);
            self.quiet();
        }

        /// The add-profile form drawing on a face, opened, with a circle
        /// chosen and the startup extrude's top cap latched by its own
        /// rows, and the selection then cleared.
        fn holding_the_top_cap() -> (Self, crate::session::FaceSelection) {
            let mut driven = Self::with_seams(Vec::new(), |app| {
                app.drafts.profile_open = true;
                app.drafts.sketch_on_face = true;
                app.drafts.profile_shape = Some(crate::forms::ShapeKind::Circle);
            });
            driven.settle();
            driven.click(crate::pane::create::ADD_FEATURE);
            let top = cap_of(pncad::prelude::CapEnd::End);
            driven.select(Selection::Face(top.clone()));
            assert_eq!(
                driven
                    .app
                    .drafts
                    .sketch_face
                    .as_ref()
                    .map(crate::tree::HeldPick::face),
                Some(&top),
                "the rows latched it"
            );
            driven.select(Selection::None);
            (driven, top)
        }
    }

    /// **The add-profile form's Cancel lets its held face go**: closed
    /// behind its door, the form holds nothing, so the viewport marks
    /// nothing as held. Goes red if `Drafts::held_face` stops reading
    /// `profile_open`.
    #[test]
    fn the_add_profile_forms_cancel_lets_its_held_face_go() {
        use crate::pickindex::IdMap;
        const NONE: u32 = IdMap::NOTHING;
        let (mut driven, top) = Driven::holding_the_top_cap();
        let top_id = driven.patch_of(&top);
        assert_eq!(driven.held(), [top_id, NONE, NONE], "held while open");
        driven.click("Cancel");
        driven.quiet();
        assert!(!driven.app.drafts.profile_open, "the Cancel closed it");
        assert_eq!(
            driven.held(),
            [NONE, NONE, NONE],
            "closed, it holds nothing"
        );
    }

    /// **Folding the section the open add-profile form sits in lets its
    /// held face go too**: the form is no more on screen than behind
    /// its door, and a face marked as held by a form nobody can see is
    /// the defect the door's gate closes.
    #[test]
    fn folding_the_add_profile_forms_section_lets_its_held_face_go() {
        use crate::pickindex::IdMap;
        const NONE: u32 = IdMap::NOTHING;
        let (mut driven, top) = Driven::holding_the_top_cap();
        let top_id = driven.patch_of(&top);
        assert_eq!(driven.held(), [top_id, NONE, NONE], "held while open");
        driven.click(crate::pane::create::ADD_FEATURE);
        driven.quiet();
        assert!(
            driven.each(crate::pane::create::ADD_PROFILE).is_empty(),
            "the section folded"
        );
        assert_eq!(
            driven.held(),
            [NONE, NONE, NONE],
            "a folded form holds nothing"
        );
    }

    /// **The add-profile form's held face is marked after the selection
    /// moves on, and only while the form holds it.**
    ///
    /// Driven through the form itself: the pick is latched by the
    /// on-face rows the Properties pane draws, not written into the
    /// drafts by this row.
    #[test]
    fn the_add_profile_forms_held_face_is_marked_until_the_form_lets_it_go() {
        use crate::pickindex::IdMap;
        use pncad::prelude::CapEnd;
        const NONE: u32 = IdMap::NOTHING;
        let (mut driven, top) = Driven::holding_the_top_cap();
        let bottom = cap_of(CapEnd::Start);
        let (top_id, bottom_id) = (driven.patch_of(&top), driven.patch_of(&bottom));
        let marked = driven.marks();
        assert_eq!(
            (marked.highlight().selected, marked.highlight().held),
            (NONE, [top_id, NONE, NONE]),
            "the held face is marked once nothing is selected"
        );

        // A second face pick moves the seat, and the mark with it.
        driven.select(Selection::Face(bottom.clone()));
        let marked = driven.marks();
        assert_eq!(
            (marked.highlight().selected, marked.highlight().held),
            (bottom_id, [bottom_id, NONE, NONE]),
            "held and selected at once"
        );
        driven.select(Selection::None);
        assert_eq!(
            driven.held(),
            [bottom_id, NONE, NONE],
            "the mark moved with the seat"
        );

        // Drawing off the face releases the pick without clearing the
        // latch; drawing on it again holds it again.
        driven.click("on the picked face");
        driven.quiet();
        assert_eq!(
            driven
                .app
                .drafts
                .sketch_face
                .as_ref()
                .map(crate::tree::HeldPick::face),
            Some(&bottom),
            "kept, not cleared"
        );
        assert_eq!(driven.held(), [NONE; 3], "a released pick is not marked");
        driven.click("on the picked face");
        driven.quiet();
        assert_eq!(driven.held(), [bottom_id, NONE, NONE], "held again");
    }

    /// **A held face on a copy a feature re-points stays held, drawn
    /// and committable.** The pick is on the placement's copy; the
    /// transform re-points that placement (A10) to a placement of the
    /// extrude moved within its own space, so the copy still draws the
    /// face — moved — and the form commits a sketch on the face of the
    /// moved body. (A held face the picture does not draw — a hidden
    /// instance's — is `tests/assembly_display.rs`'s row.)
    #[test]
    fn a_held_face_on_a_copy_a_feature_re_points_stays_held() {
        use crate::test_support::{len, scl};
        use pncad::document::{Dimension, Formula};
        let (mut driven, top) = Driven::holding_the_top_cap();
        driven.select(Selection::Face(top));
        driven.perform(SessionOp::AddTransform {
            input: extrude().into(),
            translation: [
                len(0.05),
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: Formula::zero(Dimension::Angle),
        });
        let [held, ..] = driven.held();
        assert_ne!(
            held,
            crate::pickindex::IdMap::NOTHING,
            "the moved copy still draws the held face"
        );
        assert_eq!(
            driven.marks().highlight().selected,
            held,
            "the selection follows the face onto the moved copy"
        );
        let moved = *driven
            .app
            .session
            .doc()
            .ids()
            .iter()
            .find(|&&id| {
                matches!(driven.app.session.doc().node(id), Some(Node::Place { .. }))
                    && id != placement()
            })
            .expect("the transform's placement landed");
        driven.click(crate::pane::create::ADD_PROFILE);
        driven.quiet();
        let doc = driven.app.session.doc();
        let union = doc
            .ids()
            .into_iter()
            .find(|&id| matches!(doc.node(id), Some(Node::Union { .. })));
        let union = union.expect("the button committed a sketch on the face");
        assert!(
            doc.upstream(union).contains(&moved),
            "on the moved body, which the copy places"
        );
    }

    /// **A document that replaces this one drops the form's held face**:
    /// the pick is a (node, name) of the document it was made in, and
    /// the next document's same-numbered node would be marked and
    /// committed against. Both doors that replace a document.
    #[test]
    fn a_replaced_document_drops_the_held_face() {
        let (mut driven, _top) = Driven::holding_the_top_cap();
        driven.perform(SessionOp::NewDocument {
            name: "fresh".to_owned(),
        });
        assert_eq!(driven.app.drafts.sketch_face, None, "new document");

        let (mut driven, _top) = Driven::holding_the_top_cap();
        // A folder of its own: a save scans the folder it writes into.
        let dir =
            std::env::temp_dir().join(format!("held face across open {}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let path = dir.join("doc.pncad");
        driven
            .app
            .perform_batch(vec![SessionOp::Save(path.clone())]);
        driven.perform(SessionOp::Open(path.clone()));
        let removed = std::fs::remove_dir_all(&dir);
        assert_eq!(driven.app.drafts.sketch_face, None, "open");
        removed.expect("the row removes the document it saved");
    }

    /// **The op loop re-speaks what outlives the op that made it**
    /// (`ViewerApp::respeak_held`): picked, renamed, then deleted, the
    /// open tool's drop and the add-profile form's held face both name the
    /// extrude by its last label, through `perform_batch` itself.
    #[test]
    fn the_op_loop_says_a_held_node_by_its_last_label() {
        let rename_and_delete = |driven: &mut Driven| {
            driven.perform(SessionOp::SetLabel {
                node: extrude(),
                label: label("plinth"),
            });
            driven
                .app
                .perform_batch(vec![SessionOp::DeleteNode { node: extrude() }]);
        };
        let plinth = format!(
            "Extrude \"plinth\" ({})",
            test_utils::refusal::tag(extrude().0.digest())
        );

        let mut driven = Driven::with(vec![SessionOp::SetLabel {
            node: extrude(),
            label: label("base"),
        }]);
        driven.settle();
        driven.app.tools.open(crate::tools::ToolKind::Transform);
        driven.select(Selection::Node(extrude()));
        rename_and_delete(&mut driven);
        let drawn = driven.quiet();
        let dropped = drawn
            .iter()
            .map(|(run, _)| run)
            .find(|run| run.contains("no longer in the document"))
            .unwrap_or_else(|| panic!("the drop is said: {drawn:?}"));
        assert!(
            dropped.contains(&plinth),
            "the tool's drop says the last label: {dropped}"
        );

        let (mut driven, _top) = Driven::holding_the_top_cap();
        rename_and_delete(&mut driven);
        assert_eq!(
            driven
                .app
                .drafts
                .sketch_face
                .as_ref()
                .expect("the form holds the top cap")
                .said()
                .spoken(extrude())
                .to_string(),
            plinth,
            "the form's held face says the last label"
        );
    }

    /// **A document that replaces this one starts the open tool over**:
    /// it stays open and holds nothing, since its picks are ids of the
    /// document they were made in. Both doors that replace a document;
    /// the reopened copy holds the same ids, so its pick would survive.
    #[test]
    fn a_replaced_document_drops_the_open_tools_picks() {
        let picked = || {
            let mut driven = Driven::with(Vec::new());
            driven.settle();
            driven.app.tools.open(crate::tools::ToolKind::Transform);
            driven.select(Selection::Node(extrude()));
            assert_eq!(
                driven.app.tools.transform().and_then(|tool| tool.input()),
                Some(extrude()),
                "the pick is held"
            );
            driven
        };
        let held = |driven: &Driven| {
            (
                driven.app.tools.open_kind(),
                driven.app.tools.transform().and_then(|tool| tool.input()),
            )
        };
        let mut driven = picked();
        driven.app.perform_batch(vec![SessionOp::NewDocument {
            name: "fresh".to_owned(),
        }]);
        assert_eq!(
            held(&driven),
            (Some(crate::tools::ToolKind::Transform), None),
            "new document"
        );

        let mut driven = picked();
        // A folder of its own: a save scans the folder it writes into.
        let dir =
            std::env::temp_dir().join(format!("tool picks across open {}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let path = dir.join("doc.pncad");
        driven
            .app
            .perform_batch(vec![SessionOp::Save(path.clone())]);
        driven.perform(SessionOp::Open(path.clone()));
        let removed = std::fs::remove_dir_all(&dir);
        assert_eq!(
            held(&driven),
            (Some(crate::tools::ToolKind::Transform), None),
            "open"
        );
        removed.expect("the row removes the document it saved");
    }

    /// **A replaced document is owed a re-frame and a fresh δ, and the
    /// budget's verdict about the last one is dropped**, whichever door
    /// replaced it. From a settled app whose fits are spent and which
    /// holds a verdict, each door installs [`DocumentDerived::owed`];
    /// the reopened copy's landing spends both fits again.
    #[test]
    fn a_replaced_document_is_owed_both_fits_and_drops_the_verdict() {
        let settled = || {
            let mut driven = Driven::with(Vec::new());
            driven.settle();
            let delta = driven.app.delta;
            driven.app.derived.budget_delta = Some(crate::scene::FittedDelta {
                requested_cost: Some(1),
                ..crate::scene::FittedDelta::as_requested(delta)
            });
            let derived = driven.app.derived;
            assert!(
                !derived.fit_camera && !derived.fit_delta && derived.budget_delta.is_some(),
                "the startup landing spent both fits, and a verdict is held: {derived:?}"
            );
            driven
        };

        let mut driven = settled();
        driven.app.perform_batch(vec![SessionOp::NewDocument {
            name: "fresh".to_owned(),
        }]);
        assert_eq!(driven.app.derived, DocumentDerived::owed(), "new document");

        let mut driven = settled();
        // A folder of its own: a save scans the folder it writes into.
        let dir = std::env::temp_dir().join(format!("framing across open {}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let path = dir.join("doc.pncad");
        driven
            .app
            .perform_batch(vec![SessionOp::Save(path.clone())]);
        driven
            .app
            .perform_batch(vec![SessionOp::Open(path.clone())]);
        let removed = std::fs::remove_dir_all(&dir);
        assert_eq!(driven.app.derived, DocumentDerived::owed(), "open");
        driven.settle();
        let derived = driven.app.derived;
        assert!(
            !derived.fit_camera && !derived.fit_delta,
            "the opened document's landing spends both fits: {derived:?}"
        );
        removed.expect("the row removes the document it saved");
    }

    /// The camera as a row compares it: where it looks and how far
    /// back it stands.
    fn camera_pose(app: &ViewerApp) -> ([f64; 3], f64) {
        let target = app.camera.target();
        ([target.x, target.y, target.z], app.camera.distance())
    }

    /// The camera backed off to `factor` times its distance, so a
    /// re-frame that follows is visible as a return.
    fn backed_off(app: &ViewerApp, factor: f64) -> crate::camera::Camera {
        let camera = &app.camera;
        let radius = app
            .scene
            .bounds()
            .map_or(crate::camera::DEFAULT_FRAME_RADIUS, |b| {
                ((b.max_x - b.min_x).powi(2)
                    + (b.max_y - b.min_y).powi(2)
                    + (b.max_z - b.min_z).powi(2))
                .sqrt()
                    / 2.0
            });
        crate::camera::Camera::new(
            camera.target(),
            camera.distance() * factor,
            camera.yaw(),
            camera.pitch(),
            camera.fov_y(),
            radius,
        )
        .expect("a camera backed off inside the zoom band")
    }

    /// **A replaced document re-frames the CAMERA, not only the flag**,
    /// through both doors and in sequence (Open → NewDocument → Open):
    /// a camera the user backed off returns to the reopened plate's
    /// fit, a new (empty) document frames the origin, and the next
    /// Open frames the plate again. `a_replaced_document_is_owed_both_
    /// fits_and_drops_the_verdict` holds the flags; this holds what
    /// they are for, on the frames the viewport actually draws.
    #[test]
    fn a_replaced_document_reframes_the_camera_through_both_doors() {
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let plate = camera_pose(&driven.app);
        // A folder of its own: a save scans the folder it writes into.
        let dir = std::env::temp_dir().join(format!("reframe across doors {}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let path = dir.join("doc.pncad");
        driven
            .app
            .perform_batch(vec![SessionOp::Save(path.clone())]);

        driven.app.camera = backed_off(&driven.app, 3.0);
        driven.perform(SessionOp::Open(path.clone()));
        driven.settle();
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert_eq!(camera_pose(&driven.app), plate, "open re-frames the plate");

        driven.app.camera = backed_off(&driven.app, 3.0);
        driven.perform(SessionOp::NewDocument {
            name: "fresh".to_owned(),
        });
        driven.settle();
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let (target, _) = camera_pose(&driven.app);
        assert_eq!(target, [0.0; 3], "a new document frames the origin");
        assert!(
            driven.app.derived.fit_camera,
            "its boundless landing keeps the fit owed"
        );

        driven.perform(SessionOp::Open(path.clone()));
        driven.settle();
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let removed = std::fs::remove_dir_all(&dir);
        assert_eq!(camera_pose(&driven.app), plate, "the next open re-frames");
        removed.expect("the row removes the document it saved");
    }

    /// A fit seam that records the generation of every request it is
    /// handed, then runs it as the seam it wraps would.
    struct Recorded {
        inner: Box<dyn crate::evalseam::FitService>,
        asked: std::sync::Arc<std::sync::Mutex<Vec<crate::generation::Generation>>>,
    }

    impl crate::evalseam::FitService for Recorded {
        fn submit(&mut self, request: crate::evalseam::FitRequest) {
            self.asked
                .lock()
                .expect("the record is not poisoned")
                .push(request.generation);
            self.inner.submit(request);
        }
        fn poll(&mut self) -> Option<crate::evalseam::FitDone> {
            self.inner.poll()
        }
        fn busy(&self) -> bool {
            self.inner.busy()
        }
    }

    /// **An empty new document's first body is framed, and its first
    /// body to price is budgeted.** The empty document lands a
    /// boundless scene with nothing to price, so neither fit is spent
    /// on it. The instance added next lands with both owed: its scene
    /// has bounds, so the camera is framed on it, but it is in a space
    /// of its own and the document's product is empty, so there is
    /// still nothing for the budget to price and the δ fit stays owed.
    /// The landing that first has a product is priced, once — spliced
    /// in here as the plate's session, since no op this harness drives
    /// puts a body in the world.
    #[test]
    fn an_empty_new_documents_first_body_is_framed_and_budgeted() {
        let tol = pncad::tolerance::witness();
        let (dir, part, _) = part_workspace("first body fits", tol);
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        let asked = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let inner = std::mem::replace(
            &mut driven.app.fit,
            Box::new(crate::evalseam::InlineFitter::new()),
        );
        driven.app.fit = Box::new(Recorded {
            inner,
            asked: std::sync::Arc::clone(&asked),
        });
        let asked_for = || asked.lock().expect("the record is not poisoned").clone();

        driven.perform(SessionOp::NewDocument {
            name: "first-body-fits".to_owned(),
        });
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert!(
            driven.app.scene.bounds().is_none(),
            "the fixture's empty document lands a boundless scene"
        );
        let derived = driven.app.derived;
        assert!(
            derived.fit_camera && derived.fit_delta,
            "a boundless landing spends neither fit: {derived:?}"
        );
        assert_eq!(asked_for(), Vec::new(), "nothing was priced");

        driven.app.camera = backed_off(&driven.app, 3.0);
        driven.perform(SessionOp::Save(dir.join("assembly.pncad")));
        driven.perform(SessionOp::AddInstance { id: part.id() });
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let removed = std::fs::remove_dir_all(&dir);
        assert!(
            driven.app.scene.bounds().is_some(),
            "the instance landed a scene with a body"
        );
        let derived = driven.app.derived;
        assert!(
            !derived.fit_camera && derived.fit_delta,
            "the first body spends the camera fit, and with no product the δ fit stays owed: \
             {derived:?}"
        );
        assert_eq!(asked_for(), Vec::new(), "still nothing was priced");
        let framed = camera_pose(&driven.app);
        driven.app.pending_fit = true;
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert_eq!(
            camera_pose(&driven.app),
            framed,
            "the camera already stands at the first body's fit"
        );

        let (document, _) = crate::scene::plate_with_hole(tol).expect("the startup document");
        let mut session = crate::session::DocSession::inline(document, tol);
        session.pump();
        driven.app.session = session;
        driven.quiet();
        assert!(
            !driven.app.derived.fit_delta,
            "the product's landing spent the δ fit"
        );
        assert_eq!(
            asked_for().into_iter().map(Some).collect::<Vec<_>>(),
            vec![driven.app.session.landed_generation()],
            "the budget priced that landing, once"
        );
        removed.expect("the row removes its workspace");
    }

    /// **A camera moved before the index lands is kept**: startup's
    /// fit is the first frame's (`pending_fit`, over the startup mesh),
    /// so the first indexed scene owes no second one that would undo a
    /// move made in between.
    #[test]
    fn a_camera_moved_before_the_index_lands_is_kept() {
        let mut driven = Driven::with(Vec::new());
        assert!(
            driven.app.scene_key.is_none(),
            "the move is made before the first indexed scene lands"
        );
        driven.app.camera = backed_off(&driven.app, 2.0);
        let moved = camera_pose(&driven.app);
        driven.settle();
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert!(driven.app.scene_key.is_some(), "the index has landed");
        assert_eq!(camera_pose(&driven.app), moved, "the moved camera stands");
    }

    /// **An index refused as a whole draws nothing**, never the picture
    /// from before the refusal, and says so loudly. Such a refusal is a
    /// fault of the index itself, which no document produces, so the
    /// seam here refuses every build it answers.
    #[test]
    fn a_whole_index_refusal_clears_the_picture() {
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert!(
            driven.app.scene.stats().triangles > 0 && driven.app.scene_key.is_some(),
            "the premise: the plate is drawn from an index"
        );
        let (refusing, _) =
            crate::test_support::RefusingIndexer::new(crate::evalseam::InlineIndexer::new());
        driven.app.picks = crate::pickcache::PickCache::new(Box::new(refusing));
        driven.settle();
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert!(driven.app.picks.error().is_some(), "the refusal is held");
        assert!(
            driven.app.scene.positions().is_empty(),
            "nothing is drawn under a refused index"
        );
        assert_eq!(driven.app.scene_key, None, "and no index minted its ids");
        let badge = crate::frame::index_badge(
            driven.app.picks.error(),
            driven.app.picks.index(),
            driven.app.session.landed_pair(),
        )
        .expect("the refusal badges");
        assert_eq!(badge.tone(), crate::frame::Tone::Actionable);
        assert!(
            badge
                .label()
                .ends_with("nothing is drawn or picked until the index builds"),
            "{}",
            badge.label()
        );
    }

    /// **The mate tool's two picks are marked once the selection moves
    /// on, and not once the tool closes.**
    #[test]
    fn the_mate_tools_held_picks_are_marked_until_it_closes() {
        use crate::pickindex::IdMap;
        use pncad::prelude::CapEnd;
        const NONE: u32 = IdMap::NOTHING;
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        let a = cap_of(CapEnd::End);
        let b = cap_of(CapEnd::Start);
        let (a_id, b_id) = (driven.patch_of(&a), driven.patch_of(&b));
        driven.app.tools.open(crate::tools::ToolKind::Mate);
        driven.click_through(vec![
            Selection::Face(a),
            Selection::Face(b),
            Selection::None,
        ]);
        let marked = driven.marks();
        assert_eq!(
            (marked.highlight().selected, marked.highlight().held),
            (NONE, [NONE, a_id, b_id])
        );
        driven.app.tools.close();
        driven.quiet();
        assert_eq!(driven.held(), [NONE; 3], "a closed tool holds nothing");
    }

    /// **The blend tool's held edges are drawn in the held lane**, not
    /// the selected one, once the selection moves on — and not once
    /// the tool closes.
    #[test]
    fn the_blend_tools_held_edges_are_drawn_in_the_held_lane() {
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        let edge = {
            let index = driven.on_screen();
            let id = *index
                .edges_in(placement(), 0)
                .first()
                .expect("the extrude draws edges");
            crate::session::EdgeSelection {
                name: index
                    .edge_name_of(id)
                    .expect("a drawn edge is named")
                    .clone(),
                node: placement(),
                body: 0,
            }
        };
        driven.app.tools.open(crate::tools::ToolKind::Blend);
        driven.click_through(vec![Selection::Edge(edge), Selection::None]);
        let marked = driven.marks();
        assert!(marked.edges().selected.is_empty(), "nothing is selected");
        assert!(!marked.edges().held.is_empty(), "the held edge is drawn");
        driven.app.tools.close();
        driven.quiet();
        assert!(
            driven.marks().edges().held.is_empty(),
            "a closed tool holds nothing"
        );
    }

    /// **A deselect that is not a viewport click says nothing about the
    /// open tool**: it is no attempt to seat anything, so the miss notice
    /// is the viewport's alone.
    #[test]
    fn a_deselect_outside_the_viewport_with_a_tool_open_says_nothing() {
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        driven.app.tools.open(crate::tools::ToolKind::Boolean);
        driven.select(Selection::Node(extrude()));
        driven.select(Selection::None);
        let said = driven.app.status.as_ref().map(|m| m.text().to_owned());
        assert!(
            !said
                .as_deref()
                .is_some_and(|text| text.contains("no pick taken")),
            "{said:?}"
        );
    }

    /// **A blend target whose drawn edges the index cannot all name
    /// says so**: the held mark is badged in the index's words, and the
    /// all-edges load puts the index's refusal on the line rather than
    /// saying the body has no edges. The naming layer's refusal is
    /// planted in the app's own index (`PickIndex::unname_edge`).
    #[test]
    fn a_blend_target_whose_edges_cannot_be_named_says_so() {
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        let (edge, drawn) = {
            let index = driven.on_screen();
            let drawn = index.edges_in(placement(), 0).to_vec();
            let edge = crate::session::EdgeSelection {
                name: index
                    .edge_name_of(drawn[0])
                    .expect("a drawn edge is named")
                    .clone(),
                node: placement(),
                body: 0,
            };
            (edge, drawn)
        };
        driven.app.tools.open(crate::tools::ToolKind::Blend);
        driven.click_through(vec![Selection::Edge(edge)]);
        let named = driven.quiet();
        assert!(
            !named.iter().any(|(run, _)| run.starts_with("held edges:")),
            "a held set the index names wholly is not badged: {named:?}"
        );

        let first = driven
            .app
            .picks
            .index_mut()
            .expect("the settled app holds an index")
            .unname_edge(drawn[1]);
        let refused = crate::pickindex::EdgeNamesRefused {
            node: placement(),
            body: 0,
            first,
            named: drawn.len() - 1,
            refused: 1,
        };
        let (landed, _) = driven.app.session.landed_pair().expect("the plate lands");
        let refused_said =
            pncad::document::Said(&refused, pncad::document::Speaker::of(landed)).to_string();
        assert!(
            refused_said.contains("edges it draws on Place "),
            "the refusal's node is said as the landed document holds it: {refused_said}"
        );
        let spelled = crate::frame::Spelled::in_landed(refused.clone(), Some(landed));
        let badge =
            crate::frame::held_edges_badge(Some(&spelled), Some(landed)).expect("a refusal badges");
        let badged = driven.quiet();
        assert!(
            badged.iter().any(|(run, _)| run == badge.label()),
            "the toolbar reads the held mark's refusal: {badged:?}"
        );

        // A read of the frame the viewport drew, not a latch: with the
        // viewport not drawn, the same held set and the same refusing
        // index badge nothing, and drawn again they badge again. Only
        // `held_edges_refused` is non-zero here, so the whole-value
        // asserts below guard that member alone; `profiles_undrawn`
        // waits on a fixture that makes it non-zero
        // (`work/vdoc/committed-profiles-undrawn-has-no-witness.md`).
        assert_ne!(
            driven.app.viewport_reads,
            super::ViewportReads::default(),
            "the drawn viewport found the refusal"
        );
        let viewport = driven
            .app
            .tree
            .tiles
            .find_pane(&super::Pane::Viewport)
            .expect("the layout has a viewport");
        driven.app.tree.set_visible(viewport, false);
        let undrawn = driven.quiet();
        assert!(
            !undrawn
                .iter()
                .any(|(run, _)| run.starts_with("held edges:")),
            "a viewport that did not draw leaves no badge standing: {undrawn:?}"
        );
        assert_eq!(
            driven.app.viewport_reads,
            super::ViewportReads::default(),
            "a viewport that did not draw leaves none of its reads standing"
        );
        driven.app.tree.set_visible(viewport, true);
        assert!(driven.quiet().iter().any(|(run, _)| run == badge.label()));

        driven.click(crate::pane::create::BLEND_EDGES);
        driven.click("Select all edges");
        let said = driven.quiet();
        let line = said
            .iter()
            .map(|(run, _)| run)
            .find(|run| run.contains("the tool loaded no edges:"))
            .unwrap_or_else(|| panic!("the refused load is on the line: {said:?}"));
        assert!(line.contains(&refused_said), "{line}");
        assert!(!line.contains("has no edges to select"), "{line}");
        assert_eq!(
            driven.app.tools.blend().map(crate::blend::BlendTool::count),
            Some(1),
            "the held edge stands"
        );

        // The badge is a read of this frame's held set, not a latch: with
        // the picks cleared there is nothing held to mark, and the same
        // index's refusal is no longer the toolbar's to say.
        driven.click("Clear picks");
        let cleared = driven.quiet();
        assert_eq!(
            driven.app.tools.blend().map(crate::blend::BlendTool::count),
            Some(0),
            "the picks are cleared"
        );
        assert!(
            !cleared
                .iter()
                .any(|(run, _)| run.starts_with("held edges:")),
            "a badge with nothing held is gone: {cleared:?}"
        );
    }

    /// A seam that lands a request only while `open` is set: a run held
    /// back for as long as a row wants the landed document behind the
    /// shown one.
    struct Gated {
        inner: crate::evalseam::InlineEvaluator,
        open: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }

    impl crate::evalseam::EvalService for Gated {
        fn submit(&mut self, request: crate::evalseam::EvalRequest) {
            self.inner.submit(request);
        }

        fn cancel(&mut self) {
            self.inner.cancel();
        }

        fn poll(&mut self) -> Option<crate::evalseam::EvalDone> {
            if self.open.load(std::sync::atomic::Ordering::SeqCst) {
                self.inner.poll()
            } else {
                None
            }
        }

        fn busy(&self) -> bool {
            self.inner.busy()
        }
    }

    /// The startup app over a [`Gated`] seam, open, and the gate.
    fn gated() -> (Driven, std::sync::Arc<std::sync::atomic::AtomicBool>) {
        let open = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let gate = std::sync::Arc::clone(&open);
        let driven = Driven::with_seams(Vec::new(), |app| {
            app.session = crate::session::DocSession::new(
                app.session.doc().clone(),
                pncad::tolerance::witness(),
                Box::new(Gated {
                    inner: crate::evalseam::InlineEvaluator::new(),
                    open: gate,
                }),
            );
        });
        (driven, open)
    }

    /// `text` as a node's label.
    fn label(text: &str) -> Option<pncad::document::Label> {
        Some(pncad::document::Label::new(text).expect("a label"))
    }

    /// **The rename `shown plate` of `node` committed and held behind
    /// `gate`**, two frames drawn after it: what they painted, and
    /// `node` as the landed document still says it.
    fn renamed_behind(
        driven: &mut Driven,
        gate: &std::sync::atomic::AtomicBool,
        node: RecipeNodeId,
    ) -> (Vec<(String, egui::Rect)>, String) {
        gate.store(false, std::sync::atomic::Ordering::SeqCst);
        driven.app.perform_batch(vec![SessionOp::SetLabel {
            node,
            label: label("shown plate"),
        }]);
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let drawn = driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        assert!(
            driven
                .app
                .session
                .doc()
                .spoken(node)
                .to_string()
                .contains("shown plate"),
            "the shown document is ahead of the landed one"
        );
        let landed = driven
            .app
            .session
            .landed_pair()
            .expect("the earlier run stays landed")
            .0
            .spoken(node)
            .to_string();
        assert!(landed.contains("landed plate"), "{landed}");
        (drawn, landed)
    }

    /// **The pick path speaks from the landed document, not the shown
    /// one.** A rename committed while its run is held leaves the shown
    /// document ahead of the landed one; the blend panel's target line
    /// and the held mark's badge say the label the landed run's ids
    /// were read in, which is the label the picture they describe was
    /// drawn under.
    #[test]
    fn the_pick_path_says_the_landed_label_while_a_rename_has_not_landed() {
        let (mut driven, open) = gated();
        // The pick path speaks the drawn copy: its placement.
        driven.app.perform_batch(vec![SessionOp::SetLabel {
            node: placement(),
            label: label("landed plate"),
        }]);
        driven.settle();
        let (edge, unnamed) = {
            let index = driven.on_screen();
            let drawn = index.edges_in(placement(), 0).to_vec();
            let edge = crate::session::EdgeSelection {
                name: index
                    .edge_name_of(drawn[0])
                    .expect("a drawn edge is named")
                    .clone(),
                node: placement(),
                body: 0,
            };
            (edge, drawn[1])
        };
        driven.app.tools.open(crate::tools::ToolKind::Blend);
        driven.click_through(vec![Selection::Edge(edge)]);
        driven
            .app
            .picks
            .index_mut()
            .expect("the settled app holds an index")
            .unname_edge(unnamed);
        driven.quiet();
        driven.click(crate::pane::create::BLEND_EDGES);
        driven.quiet();

        let (drawn, landed) = renamed_behind(&mut driven, &open, placement());
        for prefix in ["1 edges picked on ", "held edges:"] {
            let line = drawn
                .iter()
                .map(|(run, _)| run)
                .find(|run| run.starts_with(prefix))
                .unwrap_or_else(|| panic!("{prefix:?} is drawn: {drawn:?}"));
            assert!(
                line.contains(&landed) && !line.contains("shown plate"),
                "{prefix:?} says the landed label: {line}"
            );
        }
    }

    /// **The mate tool's same-copy refusal says the landed label while
    /// the landed document is behind**: the run that would land the
    /// rename and the delete is held, so the landed document still says
    /// the picked copy's placement by its old label and the survival
    /// step keeps the picks. The chrome speaks the landed document, so
    /// the commit's refusal says that label, as the panel line does —
    /// not the rename the shown document made since.
    #[test]
    fn the_mate_refusal_says_the_landed_label_while_a_delete_has_not_landed() {
        let (mut driven, open) = gated();
        driven.app.perform_batch(vec![SessionOp::SetLabel {
            node: placement(),
            label: label("base"),
        }]);
        driven.settle();
        driven.app.tools.open(crate::tools::ToolKind::Mate);
        driven.click_through(vec![
            Selection::Face(cap_of(pncad::prelude::CapEnd::End)),
            Selection::Face(cap_of(pncad::prelude::CapEnd::Start)),
            Selection::None,
        ]);
        open.store(false, std::sync::atomic::Ordering::SeqCst);
        driven.app.perform_batch(vec![SessionOp::SetLabel {
            node: placement(),
            label: label("plinth"),
        }]);
        driven
            .app
            .perform_batch(vec![SessionOp::DeleteNode { node: placement() }]);
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let landed = driven
            .app
            .session
            .landed_pair()
            .expect("the earlier run stays landed")
            .0
            .spoken(placement())
            .to_string();
        assert!(
            landed.contains("base"),
            "the landed document is behind: {landed}"
        );
        assert!(
            driven
                .app
                .tools
                .mate()
                .is_some_and(|tool| tool.state().picks()[0].is_some()),
            "the survival step kept the pick"
        );
        driven.click(crate::pane::create::ASSEMBLE_PARTS);
        driven.click(&crate::tools::ToolKind::Mate.commit());
        driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
        let said = status(&driven).expect("the commit refused");
        let base = format!(
            "Place \"base\" ({})",
            test_utils::refusal::tag(placement().0.digest())
        );
        // Both picks are caps of the one copy, so the commit refuses
        // the pair, naming the copy's placement.
        assert!(
            said.contains(&format!("both picks are on {base},")),
            "the refusal says the landed label: {said}"
        );
    }

    /// **An edge-on-edge direction mate committed at the panel's own
    /// default sense does not turn its copy over**: the plate duplicated
    /// beside itself, one vertical corner edge of each picked, the panel
    /// switched to `direction` and committed untouched. The default is
    /// the kind's own (`matetool::sense_offer`), not a plane's rest
    /// carried across, and the panel says what turning it round does.
    #[test]
    fn an_edge_direction_mate_at_the_panels_default_keeps_its_copy_upright() {
        let mut driven = Driven::with(Vec::new());
        driven.settle();
        driven.perform(SessionOp::Duplicate {
            input: pncad::document::Operand::Node(extrude()),
            beside: Some(placement()),
        });
        let doc = driven.app.session.committed_doc().clone();
        let [copy] = crate::world::placements_of(&doc, extrude())
            .into_iter()
            .filter(|&at| at != placement())
            .collect::<Vec<_>>()[..]
        else {
            panic!("the duplicate is one more copy")
        };
        let index = driven.on_screen();
        let vertical = |node| {
            index
                .edges_in(node, 0)
                .iter()
                .find(|&&id| {
                    let points = index.edge_polyline_for(id, &crate::display::DisplayView::none());
                    let (a, b) = (points[0], points[points.len() - 1]);
                    points.len() == 2 && (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9
                })
                .map(|&id| index.edge_name_of(id).expect("named").clone())
                .expect("the plate draws a vertical edge")
        };
        let name = vertical(copy);
        let edge = |node| {
            Selection::Edge(crate::session::EdgeSelection {
                name: name.clone(),
                node,
                body: 0,
            })
        };
        driven.app.tools.open(crate::tools::ToolKind::Mate);
        driven.click_through(vec![edge(copy), edge(placement()), Selection::None]);
        driven.click(crate::pane::create::ASSEMBLE_PARTS);
        driven.click("direction");
        let shown: Vec<String> = driven.quiet().into_iter().map(|(text, _)| text).collect();
        assert!(
            shown
                .iter()
                .any(|t| t == "turned round: the two run opposite ways")
                && !shown.iter().any(|t| t == "faces rest against each other"),
            "the direction's sense control says what it does: {shown:?}"
        );
        driven.click(&crate::tools::ToolKind::Mate.commit());
        driven.quiet();
        let (doc, ev) = driven.app.session.landed_pair().expect("landed");
        assert!(
            matches!(doc.node(copy), Some(Node::Place { constraints, .. })
                if constraints.iter().any(|c| c.mate().is_some())),
            "the mate commits: {:?}",
            status(&driven)
        );
        let at = pncad::document::SolvedPoses::of(doc, ev)
            .relative(copy)
            .expect("the copy is solved");
        assert!(
            at.columns[2][2] > 0.999,
            "the copy stands upright, not turned over: {at:?}"
        );
    }

    /// **The selection's verdict speaks from the landed document, not
    /// the shown one**: the standing is asked of the landed run, so a
    /// picked face that no longer resolves names its minting node by
    /// the label that run's ids were read in while a rename is held.
    #[test]
    fn the_selections_verdict_says_the_landed_label_while_a_rename_has_not_landed() {
        let (mut driven, open) = gated();
        driven.app.perform_batch(vec![SessionOp::SetLabel {
            node: extrude(),
            label: label("landed plate"),
        }]);
        driven.settle();
        // A cap of a cap: a name the extrude never minted, held on the
        // extrude itself, whose verdict names the extrude.
        let gone = crate::session::FaceSelection {
            name: pncad::prelude::StableName {
                kind: pncad::prelude::EntityKind::Face,
                node: extrude(),
                path: vec![pncad::prelude::RoleSeg::Cap(pncad::prelude::CapEnd::End); 2],
            },
            node: extrude(),
            body: 0,
        };
        driven
            .app
            .perform_batch(vec![SessionOp::Select(Selection::Face(gone))]);
        driven.quiet();

        let (drawn, landed) = renamed_behind(&mut driven, &open, extrude());
        let line = drawn
            .iter()
            .map(|(run, _)| run)
            .find(|run| run.starts_with("this face no longer resolves in this evaluation: "))
            .unwrap_or_else(|| panic!("the verdict is drawn: {drawn:?}"));
        assert!(
            line.contains(&landed) && !line.contains("shown plate"),
            "the verdict says the landed label: {line}"
        );
    }

    /// **The add-profile form's traced path, driven through the whole
    /// app** (`SessionOp::AddDrawnProfile`): on the cylinder of the
    /// corpus's `boss_tangent_by_value`, a boss footprint drawn in the
    /// plane of a cap runs onto the wall's trace and on along it.
    mod trace_tool {
        use pncad::document::{
            DrawnOn, LoopProgram, Node, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId,
            SlotId, StepArg, TraceCrossing, TracePassage, TraceSense, TraceStepFault,
        };
        use pncad::geom::SurfaceKind;
        use pncad::geom_core::Point2;
        use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
        use pncad::profile::{Step, Target};
        use pncad::select::{all_faces, all_vertices, vertex_position};

        use super::Driven;
        use crate::forms::ShapeKind;
        use crate::pane::create::{ADD_PROFILE, USE_THE_PICKED_FACE};
        use crate::session::{DocSession, FaceSelection, Selection, SessionOp, SketchTraceFault};
        use crate::test_support::{inserted, len, placed};

        /// The cylinder's radius, height, and the boss's reach, back
        /// wall and height, as `boss_tangent_by_value` types them.
        const RADIUS: f64 = 0.5;
        const HEIGHT: f64 = 1.0;
        const REACH: f64 = 1.0;
        const BACK: f64 = -0.25;
        const BOSS_H: f64 = 0.5;

        /// The corpus scene's cylinder, its disc centred `cx` along its
        /// body's `x`, placed in the world: the session, unevaluated
        /// until `pump`ed, its disc and the cylinder.
        fn unpumped(cx: f64) -> (DocSession, RecipeNodeId, RecipeNodeId) {
            let tol = pncad::tolerance::witness();
            let doc = pncad::document::Doc::empty_derived("boss_tangent_by_value", tol);
            let (doc, disc) = inserted(
                &doc,
                Node::Profile(ProfileProgram {
                    loops: vec![LoopProgram::circle(cx, 0.0, RADIUS).expect("a disc")],
                    ids: Vec::new(),
                    on: None,
                }),
                tol,
            );
            let (doc, cylinder) = inserted(
                &doc,
                Node::Extrude {
                    profile: disc.into(),
                    distance: len(HEIGHT),
                },
                tol,
            );
            let (doc, _) = placed(&doc, cylinder, tol);
            (DocSession::inline(doc, tol), disc, cylinder)
        }

        /// [`unpumped`], evaluated.
        fn cylinder(cx: f64) -> (DocSession, RecipeNodeId, RecipeNodeId) {
            let (mut session, disc, cylinder) = unpumped(cx);
            session.pump();
            (session, disc, cylinder)
        }

        /// The cylinder's face on a carrier of `kind`, as a pick on it:
        /// of planes, its cap at `z` (`0`, its start, or its height, its
        /// end), named by the extrude's role; its cylinder, the first
        /// of the two half-walls the disc's split circle sweeps.
        fn face(
            session: &DocSession,
            cylinder: RecipeNodeId,
            kind: SurfaceKind,
            z: f64,
        ) -> FaceSelection {
            let cap = |end| StableName {
                kind: EntityKind::Face,
                node: cylinder,
                path: vec![RoleSeg::Cap(end)],
            };
            let name = match kind {
                SurfaceKind::Plane if z == 0.0 => cap(CapEnd::Start),
                SurfaceKind::Plane => cap(CapEnd::End),
                SurfaceKind::Cylinder => {
                    let (_, ev) = session.landed_pair().expect("the cylinder landed");
                    let caps = [cap(CapEnd::Start), cap(CapEnd::End)];
                    let walls: Vec<_> = all_faces(ev, cylinder)
                        .into_iter()
                        .filter(|name| !caps.contains(name))
                        .collect();
                    let [wall, _] = walls.as_slice() else {
                        panic!("two half-walls besides the caps: {walls:?}")
                    };
                    wall.clone()
                }
                other => unreachable!("the cylinder has no {other:?} face"),
            };
            FaceSelection {
                name,
                node: cylinder,
                body: 0,
            }
        }

        /// The wall and the cap at `z`.
        fn wall_and_cap(
            session: &DocSession,
            cylinder: RecipeNodeId,
            z: f64,
        ) -> [FaceSelection; 2] {
            [
                face(session, cylinder, SurfaceKind::Cylinder, 0.0),
                face(session, cylinder, SurfaceKind::Plane, z),
            ]
        }

        /// The boss's footprint as the path editor holds it: up from
        /// `(0, BACK)`, `REACH` along the heading the trace gives, down
        /// to `BACK` and home. The trace's two steps go after row 2.
        fn footprint() -> Vec<Step<f64>> {
            vec![
                Step::At(Point2::new(0.0, BACK)),
                Step::Toward { dx: 0.0, dy: 1.0 },
                Step::Line(REACH),
                Step::LineTo(Target::Point(Point2::new(REACH, BACK))),
                Step::LineTo(Target::Start),
            ]
        }

        /// **The form, its latches taken from picks of `plane` and
        /// `traced`, and the heading against the trace.**
        fn drive(session: DocSession, plane: FaceSelection, traced: FaceSelection) -> Driven {
            let mut pane = Driven::with_seams(Vec::new(), |app| app.session = session);
            let drafts = &mut pane.app.drafts;
            drafts.profile_open = true;
            drafts.profile_shape = Some(ShapeKind::Path);
            drafts.profile_path = footprint();
            drafts.trace_on = true;
            drafts.trace_gesture.after = 1;
            pane.settle();
            pane.click(crate::pane::create::ADD_FEATURE);
            for (nth, face) in [plane, traced].into_iter().enumerate() {
                pane.app
                    .perform_batch(vec![SessionOp::Select(Selection::Face(face))]);
                let at = pane.each(USE_THE_PICKED_FACE);
                assert_eq!(at.len(), 2, "one button per latch");
                pane.click_at(at[nth]);
            }
            pane.click("against");
            pane.quiet();
            pane
        }

        /// The profile the form added: the document's last drawn one.
        fn added(pane: &Driven) -> (RecipeNodeId, ProfileProgram) {
            let doc = pane.app.session.committed_doc();
            doc.ids()
                .into_iter()
                .rev()
                .find_map(|id| match doc.node(id) {
                    Some(Node::Profile(program)) if program.on.is_some() => {
                        Some((id, program.clone()))
                    }
                    _ => None,
                })
                .expect("the form added a drawn profile")
        }

        /// Add clicked, and the profile extruded `BOSS_H`: the boss.
        fn boss(pane: &mut Driven, cylinder: RecipeNodeId) -> RecipeNodeId {
            pane.click(ADD_PROFILE);
            pane.quiet();
            let (profile, _) = added(pane);
            pane.app.perform_batch(vec![SessionOp::AddExtrude {
                profile,
                distance: len(BOSS_H),
            }]);
            pane.quiet();
            let doc = pane.app.session.committed_doc();
            doc.ids()
                .into_iter()
                .rev()
                .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })) && id != cylinder)
                .expect("the boss extrude")
        }

        /// `node`'s least `x`, greatest `y` and least `z`, as it landed.
        fn extent(pane: &Driven, node: RecipeNodeId) -> [f64; 3] {
            let (_, ev) = pane.app.session.landed_pair().expect("landed");
            assert!(
                ev.node_error(node).is_none(),
                "{node:?} builds: {:?}",
                ev.node_error(node).map(|e| e.kind.to_string())
            );
            let at: Vec<_> = all_vertices(ev, node)
                .iter()
                .map(|v| vertex_position(ev, node, v).expect("a vertex"))
                .collect();
            [
                at.iter().map(|p| p.x).fold(f64::INFINITY, f64::min),
                at.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max),
                at.iter().map(|p| p.z).fold(f64::INFINITY, f64::min),
            ]
        }

        /// `value` written into `node`'s slot `wanted` picks.
        fn set(
            pane: &mut Driven,
            node: RecipeNodeId,
            wanted: impl Fn(&SlotId) -> bool,
            value: f64,
        ) {
            let slot = crate::props::slot_rows(pane.app.session.committed_doc(), node)
                .into_iter()
                .map(|row| row.slot)
                .find(|slot| wanted(slot))
                .expect("the node carries the slot");
            pane.app.perform_batch(vec![SessionOp::SetSlot {
                node,
                slot,
                value: crate::props::SlotValue::Continuous(value),
            }]);
            pane.quiet();
        }

        fn arg(want: StepArg) -> impl Fn(&SlotId) -> bool {
            move |slot| matches!(slot, SlotId::Profile { arg, .. } if *arg == want)
        }

        fn near(a: f64, b: f64, what: &str) {
            assert!((a - b).abs() < 1e-9, "{what}: {a} against {b}");
        }

        /// **The gesture writes the document's trace steps, and the boss
        /// follows the wall.** The trace shown is the wall's circle, its
        /// tangent across the radius. The program is drawn on a frame and
        /// traces one face; after the authored heading its chain runs
        /// `line_to` the first crossing out of the wall and heads
        /// `along_trace` against it, so the one direction written is the
        /// authored one. Extruded, the boss's wall stands at the radius;
        /// the radius typed again, it stands at the new one. The door's
        /// proof of the tangency, and the frame on the top cap and off
        /// the origin, are `editor-core`'s rows
        /// (`intent_s6_e_traced_frame`), where the frame is read.
        #[test]
        fn the_gesture_writes_the_trace_steps_and_the_boss_follows_the_wall() {
            let (session, disc, cylinder) = cylinder(0.0);
            let [wall, bottom] = wall_and_cap(&session, cylinder, 0.0);
            let mut pane = drive(session, bottom, wall);
            let shown = pane
                .app
                .drafts
                .trace_shown
                .as_ref()
                .expect("the trace is read");
            let marks = shown.read.as_ref().expect("the cap's plane cuts the wall");
            assert!(!marks.marks.is_empty(), "the trace draws points");
            for mark in &marks.marks {
                let (p, t) = (mark.at, mark.tangent);
                assert!(
                    ((p.x * p.x + p.y * p.y).sqrt() - RADIUS).abs() < 1e-9
                        && (p.x * t.x + p.y * t.y).abs() < 1e-9,
                    "a point of the circle, its tangent across the radius: {mark:?}"
                );
            }
            let boss = boss(&mut pane, cylinder);
            let (_, program) = added(&pane);
            let Some(DrawnOn { faces, .. }) = &program.on else {
                unreachable!("found drawn")
            };
            assert_eq!(faces.len(), 1, "one traced face");
            let [LoopProgram::Chain(steps)] = program.loops.as_slice() else {
                panic!("one chain: {:?}", program.loops)
            };
            assert!(
                matches!(
                    steps[2],
                    ProgramStep::LineTo(ProgramTarget::OnTrace { face: 0, crossing })
                        if crossing == TraceCrossing::first(TracePassage::Leave)
                ) && matches!(
                    steps[3],
                    ProgramStep::AlongTrace {
                        sense: TraceSense::Against
                    }
                ),
                "the trace's two steps follow row 2: {steps:?}"
            );
            assert_eq!(
                steps
                    .iter()
                    .filter(|s| matches!(s, ProgramStep::Toward { .. } | ProgramStep::Angle(_)))
                    .count(),
                1,
                "the one heading written is the authored one: {steps:?}"
            );
            near(
                extent(&pane, boss)[1],
                RADIUS,
                "the boss's wall is at the radius",
            );

            set(&mut pane, disc, arg(StepArg::Radius), 0.625);
            near(
                extent(&pane, boss)[1],
                0.625,
                "the boss's wall follows the radius",
            );
        }

        /// **A traced face with no axis is refused in the pane, Add is
        /// held, and nothing is written; a traced wall is then shown,
        /// and its profile undoes, redoes and persists.** The top cap
        /// traced from the bottom cap's plane (which misses it) has no
        /// axis to stand the frame on: the pane says the frame's own
        /// refusal and Add writes nothing. Moved to the wall, the trace
        /// shows and still nothing is written until Add, which is one
        /// undoable step and reopens from a save as it was written.
        #[test]
        fn a_trace_refused_in_the_pane_writes_nothing_and_an_added_one_persists() {
            let (session, _, cylinder) = cylinder(0.0);
            let [wall, bottom] = wall_and_cap(&session, cylinder, 0.0);
            let top = face(&session, cylinder, SurfaceKind::Plane, HEIGHT);
            let before = session.committed_doc().clone();
            let undo = session.history().can_undo();
            let mut pane = drive(session, bottom, top);
            let shown = pane.app.drafts.trace_shown.as_ref().expect("read");
            let Err(fault @ SketchTraceFault::Frame(_)) = &shown.read else {
                panic!("the frame refuses: {:?}", shown.read)
            };
            let said = format!("the trace: {fault}");
            let painted = pane.quiet();
            assert!(
                painted.iter().any(|(run, _)| *run == said),
                "the pane says {said:?}: {painted:?}"
            );
            pane.click(ADD_PROFILE);
            pane.quiet();
            assert!(*pane.app.session.committed_doc() == before, "Add is held");

            let doc = pane.app.session.doc().clone();
            crate::drafts::latch(&mut pane.app.drafts.trace_face, &doc, &wall);
            pane.quiet();
            assert!(
                pane.app
                    .drafts
                    .trace_shown
                    .as_ref()
                    .is_some_and(|s| s.read.is_ok()),
                "the wall's trace shows"
            );
            assert!(
                *pane.app.session.committed_doc() == before,
                "a shown trace writes nothing"
            );
            assert_eq!(pane.app.session.history().can_undo(), undo);
            pane.click(ADD_PROFILE);
            pane.quiet();
            let after = pane.app.session.committed_doc().clone();
            assert!(after != before, "Add writes the profile");
            pane.app.perform_batch(vec![SessionOp::Undo]);
            pane.quiet();
            assert!(
                *pane.app.session.committed_doc() == before,
                "one undo takes it back"
            );
            pane.app.perform_batch(vec![SessionOp::Redo]);
            pane.quiet();
            assert!(
                *pane.app.session.committed_doc() == after,
                "and redo restores it"
            );
            let dir = std::env::temp_dir().join(format!("trace-tool-{}", std::process::id()));
            std::fs::create_dir_all(&dir).expect("a scratch directory");
            let path = dir.join("doc.pncad");
            let tol = pane.app.session.tol();
            crate::docio::save(&path, pane.app.session.committed_doc(), tol).expect("saves");
            let reopened = crate::docio::open(&path, tol).expect("reopens");
            assert!(*reopened.doc() == after, "a save reopens as written");
        }

        /// **A folded form reads no trace**: the trace is read for the
        /// form only while it is on screen, so folding the section over
        /// the open form drops the trace it showed, as its Cancel does.
        #[test]
        fn a_folded_add_profile_form_reads_no_trace() {
            let (session, _, cylinder) = cylinder(0.0);
            let [wall, bottom] = wall_and_cap(&session, cylinder, 0.0);
            let mut pane = drive(session, bottom, wall);
            assert!(pane.app.drafts.trace_shown.is_some(), "read while open");
            pane.click(crate::pane::create::ADD_FEATURE);
            pane.quiet();
            assert!(pane.app.drafts.profile_open, "folded, not closed");
            assert!(
                pane.app.drafts.trace_shown.is_none(),
                "a folded form reads no trace"
            );
        }

        /// **The wall picked for the plane is no plane**: the pane says
        /// so in the frame's words.
        #[test]
        fn a_wall_picked_for_the_plane_is_refused_as_no_plane() {
            let (session, _, cylinder) = cylinder(0.0);
            let [wall, _] = wall_and_cap(&session, cylinder, 0.0);
            let pane = drive(session, wall.clone(), wall);
            let shown = pane.app.drafts.trace_shown.as_ref().expect("read");
            assert!(
                matches!(
                    &shown.read,
                    Err(SketchTraceFault::Frame(fault))
                        if matches!(**fault, pncad::trace::TracedFrameFault::NotAPlane {
                            carrier: SurfaceKind::Cylinder
                        })
                ),
                "{:?}",
                shown.read
            );
        }

        /// **A crossing the piece does not make builds, and refuses on
        /// the profile naming it**: the piece up from inside the disc
        /// enters the wall never.
        #[test]
        fn a_crossing_the_piece_does_not_make_refuses_on_the_profile() {
            let crossing = TraceCrossing::first(TracePassage::Enter);
            let (session, _, cylinder) = cylinder(0.0);
            let [wall, bottom] = wall_and_cap(&session, cylinder, 0.0);
            let mut pane = drive(session, bottom, wall);
            pane.app.drafts.trace_gesture.crossing = crossing;
            pane.click(ADD_PROFILE);
            pane.quiet();
            let (profile, _) = added(&pane);
            let (_, ev) = pane.app.session.landed_pair().expect("landed");
            assert!(
                matches!(
                    ev.node_error(profile).map(|e| &e.kind),
                    Some(pncad::document::NodeErrorKind::Trace(refusal))
                        if refusal.fault == TraceStepFault::NoSuchCrossing { crossing, found: 0 }
                ),
                "{:?}",
                ev.node_error(profile).map(|e| e.kind.to_string())
            );
        }

        /// **Nothing landed, no trace**: a session that has evaluated
        /// nothing says so rather than reading a stale evaluation.
        #[test]
        fn a_trace_before_any_landing_is_unevaluated() {
            let (probe, _, cylinder) = cylinder(0.0);
            let [wall, bottom] = wall_and_cap(&probe, cylinder, 0.0);
            let (session, _, _) = unpumped(0.0);
            assert!(matches!(
                session.sketch_trace(&bottom, &wall),
                Err(SketchTraceFault::Unevaluated)
            ));
        }
    }

    /// Review of PR 6144: probe rows over the held driven profile.
    mod review_6144 {
        use eframe::egui;
        use pncad::document::{Dimension, FreeVar, RecipeNodeId, SlotId, StepArg, VarId, VarName};
        use pncad::geom_core::Point2;
        use pncad::profile::{Step, Target};

        use super::{Driven, startup_node};
        use crate::session::{Selection, SessionOp};

        fn profile() -> RecipeNodeId {
            startup_node(0)
        }

        fn outline(step: u32, arg: StepArg) -> SlotId {
            SlotId::Profile {
                loop_: 0,
                step,
                arg,
            }
        }

        /// The startup plate with both far corners' x driven by `w`.
        fn driven_plate() -> Driven {
            let mut driven = Driven::with(vec![
                SessionOp::DeclareVar {
                    name: VarName::from_static("w"),
                    value: FreeVar::continuous(Dimension::Length, 0.07),
                },
                SessionOp::SetSlotExpression {
                    node: profile(),
                    slot: outline(1, StepArg::TargetX),
                    text: "w".to_owned(),
                },
                SessionOp::SetSlotExpression {
                    node: profile(),
                    slot: outline(2, StepArg::TargetX),
                    text: "w".to_owned(),
                },
                SessionOp::Select(Selection::Node(profile())),
            ]);
            driven.quiet();
            driven
        }

        fn w(driven: &Driven) -> VarId {
            driven
                .app
                .session
                .committed_doc()
                .var_named("w")
                .expect("w declared")
        }

        fn perform(driven: &mut Driven, op: SessionOp) {
            let out = driven.app.session.perform(op);
            assert!(out.refusal.is_none(), "{:?}", out.refusal);
        }

        fn held_point(driven: &Driven, step: usize) -> Point2<f64> {
            match driven
                .app
                .drafts
                .profile_edit
                .as_ref()
                .expect("held")
                .loops()[0][step]
            {
                Step::At(p) | Step::LineTo(Target::Point(p)) => p,
                other => panic!("{other:?}"),
            }
        }

        /// The x the viewport draws for the outline's `step`, derived as
        /// `ViewerApp::update` derives it: the draft's number while the
        /// edit door previews, else the landed document's reading.
        fn drawn_x(driven: &Driven, step: usize) -> f64 {
            if driven.app.drafts.profile_edit.is_some() {
                held_point(driven, step).x
            } else {
                driven
                    .row(profile(), outline(step as u32, StepArg::TargetX))
                    .value
                    .expect("reads")
                    .one()
                    .expect("one value")
                    .as_f64()
            }
        }

        fn touch_far_y(driven: &mut Driven, y: f64) {
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .steps_mut(0)[3] = Step::LineTo(Target::Point(Point2::new(0.0, y)));
            driven.quiet();
        }

        fn apply(driven: &mut Driven) -> Vec<String> {
            let before = driven.app.session.history().len();
            let pre: Vec<String> = driven.quiet().into_iter().map(|(r, _)| r).collect();
            if !pre.iter().any(|r| r == "Apply") {
                eprintln!("NO APPLY; runs: {:?}", &pre[pre.len().saturating_sub(60)..]);
                return Vec::new();
            }
            driven.click("Apply");
            driven.settle();
            let painted = driven.quiet();
            eprintln!(
                "apply: history {} -> {}; status {:?}",
                before,
                driven.app.session.history().len(),
                driven.app.status.as_ref().map(|m| m.text().to_owned())
            );
            painted
                .into_iter()
                .map(|(r, _)| r)
                .filter(|r| !pre.contains(r))
                .collect()
        }

        fn source(driven: &Driven, slot: SlotId) -> Option<String> {
            crate::props::slot_rows(driven.app.session.committed_doc(), profile())
                .into_iter()
                .find(|row| row.slot == slot)
                .and_then(|row| row.source)
        }

        fn shrink_hole(driven: &mut Driven) {
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .steps_mut(1)[0] = Step::Circle {
                centre: Point2::new(0.04, 0.01),
                radius: 0.002,
            };
        }

        /// Apply's own formation (`edit_door_ui`), performed, for a pane
        /// too tall for the harness window.
        fn apply_direct(driven: &mut Driven) -> Option<String> {
            let session = &driven.app.session;
            let rows = crate::props::slot_rows(session.committed_doc(), profile());
            let noted = |slot: SlotId| rows.iter().find(|row| row.slot == slot)?.unit;
            let edit = driven.app.drafts.profile_edit.as_ref().expect("held");
            let op = SessionOp::EditProfile {
                node: profile(),
                base: edit.base().clone(),
                loops: edit
                    .programs_noted(crate::props::Notation::CANONICAL, &noted)
                    .expect("lowers"),
                ids: edit.ids(),
            };
            let out = driven.app.session.perform(op);
            driven.quiet();
            out.refusal.map(|r| r.to_string())
        }

        #[test]
        fn delete_var_then_apply() {
            let mut driven = driven_plate();
            touch_far_y(&mut driven, 0.05);
            let var = w(&driven);
            let out = driven.app.session.perform(SessionOp::DeleteVar { var });
            eprintln!(
                "delete: {:?} committed {:?}",
                out.refusal,
                out.committed.len()
            );
            driven.quiet();
            eprintln!(
                "after delete: source step2 {:?}, far y {}",
                source(&driven, outline(2, StepArg::TargetX)),
                held_point(&driven, 3).y
            );
            let said = apply(&mut driven);
            eprintln!(
                "said {said:?}; source step2 {:?}",
                source(&driven, outline(2, StepArg::TargetX))
            );
        }

        #[test]
        fn a_driven_step_the_edit_deletes() {
            let mut driven = driven_plate();
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .edit_row(0, crate::drafts::RowEdit::Remove(2));
            shrink_hole(&mut driven);
            driven.quiet();
            let said = apply_direct(&mut driven);
            eprintln!("deletes said {said:?}");
            let rows = crate::props::slot_rows(driven.app.session.committed_doc(), profile());
            for row in rows
                .iter()
                .filter(|r| matches!(r.slot, SlotId::Profile { loop_: 0, .. }))
            {
                eprintln!(
                    "  {} = {:?} src {:?}",
                    row.slot.label(),
                    row.value
                        .as_ref()
                        .map(|v| v.one().expect("one value").as_f64()),
                    row.source
                );
            }
            assert_eq!(
                source(&driven, outline(1, StepArg::TargetX)).as_deref(),
                Some("w")
            );
        }

        #[test]
        fn drivers_across_loops() {
            let mut driven = driven_plate();
            let rows = crate::props::slot_rows(driven.app.session.committed_doc(), profile());
            let hole: Vec<SlotId> = rows
                .iter()
                .map(|r| r.slot)
                .filter(|s| matches!(s, SlotId::Profile { loop_: 1, .. }))
                .collect();
            eprintln!(
                "hole slots {:?}",
                hole.iter().map(|s| s.label()).collect::<Vec<_>>()
            );
            let last = *hole.last().expect("a hole slot");
            perform(
                &mut driven,
                SessionOp::SetSlotExpression {
                    node: profile(),
                    slot: last,
                    text: "w/5".to_owned(),
                },
            );
            driven.quiet();
            touch_far_y(&mut driven, 0.05);
            let said = apply_direct(&mut driven);
            eprintln!("across said {said:?}");
            assert_eq!(source(&driven, last).as_deref(), Some("w / 5.0"));
            eprintln!(
                "hole src {:?}, step1 {:?} step2 {:?}",
                source(&driven, last),
                source(&driven, outline(1, StepArg::TargetX)),
                source(&driven, outline(2, StepArg::TargetX))
            );
            assert!(source(&driven, last).is_some());
            assert_eq!(
                source(&driven, outline(2, StepArg::TargetX)).as_deref(),
                Some("w")
            );
        }

        fn arc_plate(mode_after: pncad::profile::ArcMode) -> (Vec<String>, Option<String>) {
            use pncad::profile::{ArcData, ArcMode};
            let mut driven = Driven::with(vec![
                SessionOp::DeclareVar {
                    name: VarName::from_static("w"),
                    value: FreeVar::continuous(Dimension::Length, 0.07),
                },
                SessionOp::Select(Selection::Node(profile())),
            ]);
            driven.quiet();
            let arc = |mode| match sketch_fresh(mode) {
                ArcData::Bulge { b, .. } => ArcData::Bulge {
                    target: Target::Point(Point2::new(0.06, 0.0)),
                    b,
                },
                ArcData::Center { winding, .. } => ArcData::Center {
                    c: Point2::new(0.03, -0.01),
                    winding,
                    target: Target::Point(Point2::new(0.06, 0.0)),
                },
                ArcData::Via { .. } => ArcData::Via {
                    q: Point2::new(0.03, -0.005),
                    target: Target::Point(Point2::new(0.06, 0.0)),
                },
                other => other,
            };
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .steps_mut(0)[1] = Step::ArcTo(arc(ArcMode::Bulge));
            driven.quiet();
            let said = apply(&mut driven);
            eprintln!("arc applied: {said:?}");
            perform(
                &mut driven,
                SessionOp::SetSlotExpression {
                    node: profile(),
                    slot: outline(1, StepArg::TargetX),
                    text: "w".to_owned(),
                },
            );
            driven.quiet();
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .steps_mut(0)[1] = Step::ArcTo(arc(mode_after));
            driven.quiet();
            eprintln!(
                "remode ids {:?}",
                driven.app.drafts.profile_edit.as_ref().expect("held").ids()
            );
            let said = apply_direct(&mut driven).into_iter().collect();
            (said, source(&driven, outline(1, StepArg::TargetX)))
        }

        fn sketch_fresh(mode: pncad::profile::ArcMode) -> pncad::profile::ArcData<f64> {
            crate::sketch::fresh_arc(mode)
        }

        #[test]
        fn a_driven_arc_target_remoded_to_via() {
            let (said, src) = arc_plate(pncad::profile::ArcMode::Via);
            eprintln!("via: said {said:?} src {src:?}");
            assert_eq!(src.as_deref(), Some("w"));
        }

        #[test]
        fn a_driven_arc_target_remoded_to_radius() {
            let (said, src) = arc_plate(pncad::profile::ArcMode::Radius);
            eprintln!("radius: said {said:?} src {src:?}");
        }

        /// Claim 2: a mixed-unit point, moved, lands as the field shows.
        #[test]
        fn a_mixed_unit_point_moved_lands_exactly() {
            let mut driven = Driven::with(vec![
                SessionOp::SetSlotUnit {
                    node: profile(),
                    slot: outline(1, StepArg::TargetX),
                    unit: pncad::quantity::MM.def(),
                },
                SessionOp::SetSlotUnit {
                    node: profile(),
                    slot: outline(1, StepArg::TargetY),
                    unit: pncad::quantity::CM.def(),
                },
                SessionOp::Select(Selection::Node(profile())),
            ]);
            let painted = driven.quiet();
            eprintln!(
                "pickers: {:?}",
                painted.iter().filter(|(r, _)| r == "mixed").count()
            );
            let mut bad = Vec::new();
            for (i, (x, y)) in [(0.0613, 0.0007), (0.061_234_567, 0.001_1)]
                .into_iter()
                .enumerate()
            {
                driven
                    .app
                    .drafts
                    .profile_edit
                    .as_mut()
                    .expect("held")
                    .steps_mut(0)[1] = Step::LineTo(Target::Point(Point2::new(x, y)));
                driven.quiet();
                let said = apply(&mut driven);
                let rx = driven.row(profile(), outline(1, StepArg::TargetX));
                let ry = driven.row(profile(), outline(1, StepArg::TargetY));
                let (lx, ly) = (
                    rx.value
                        .as_ref()
                        .map(|v| v.one().expect("one value").as_f64())
                        .ok(),
                    ry.value
                        .as_ref()
                        .map(|v| v.one().expect("one value").as_f64())
                        .ok(),
                );
                eprintln!(
                    "{i}: said {said:?} x {lx:?} {:?} y {ly:?} {:?}",
                    rx.unit, ry.unit
                );
                if lx != Some(x) || ly != Some(y) {
                    bad.push((x, y, lx, ly));
                }
                let held = held_point(&driven, 1);
                eprintln!("   reloaded held {:?}", held);
            }
            assert!(bad.is_empty(), "{bad:?}");
        }
        #[test]
        fn only_the_deleted_step_is_driven() {
            let mut driven = Driven::with(vec![
                SessionOp::DeclareVar {
                    name: VarName::from_static("w"),
                    value: FreeVar::continuous(Dimension::Length, 0.07),
                },
                SessionOp::SetSlotExpression {
                    node: profile(),
                    slot: outline(2, StepArg::TargetX),
                    text: "w".to_owned(),
                },
                SessionOp::Select(Selection::Node(profile())),
            ]);
            driven.quiet();
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .edit_row(0, crate::drafts::RowEdit::Remove(2));
            shrink_hole(&mut driven);
            driven.quiet();
            let said = apply_direct(&mut driven);
            eprintln!("only-deleted said {said:?}");
        }

        #[test]
        fn driven_step1_remove_step3() {
            let mut driven = Driven::with(vec![
                SessionOp::DeclareVar {
                    name: VarName::from_static("w"),
                    value: FreeVar::continuous(Dimension::Length, 0.07),
                },
                SessionOp::SetSlotExpression {
                    node: profile(),
                    slot: outline(1, StepArg::TargetX),
                    text: "w".to_owned(),
                },
                SessionOp::Select(Selection::Node(profile())),
            ]);
            driven.quiet();
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .edit_row(0, crate::drafts::RowEdit::Remove(3));
            shrink_hole(&mut driven);
            driven.quiet();
            let said = apply_direct(&mut driven);
            eprintln!(
                "step1-driven remove3 said {said:?} src {:?}",
                source(&driven, outline(1, StepArg::TargetX))
            );
        }

        #[test]
        fn control_remove_a_step_no_driver() {
            let mut driven = Driven::with(vec![SessionOp::Select(Selection::Node(profile()))]);
            driven.quiet();
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .edit_row(0, crate::drafts::RowEdit::Remove(2));
            let painted: Vec<String> = driven.quiet().into_iter().map(|(r, _)| r).collect();
            eprintln!(
                "control differs {} runs tail {:?}",
                driven
                    .app
                    .drafts
                    .profile_edit
                    .as_ref()
                    .expect("held")
                    .differs_from_base(),
                &painted[painted.len().saturating_sub(25)..]
            );
            let said = apply(&mut driven);
            eprintln!("control said {said:?}");
        }

        /// E2E: open, edit, Apply, drag the variable, release, undo.
        #[test]
        fn e2e_apply_then_drag_then_release_then_undo() {
            let mut driven = driven_plate();
            assert_eq!(held_point(&driven, 2).x, 0.07);
            touch_far_y(&mut driven, 0.05);
            let states = driven.app.session.history().len();
            driven.click("Apply");
            driven.settle();
            driven.quiet();
            eprintln!(
                "history {} -> {}",
                states,
                driven.app.session.history().len()
            );
            for step in [1, 2] {
                let row = driven.row(profile(), outline(step, StepArg::TargetX));
                assert_eq!(row.source.as_deref(), Some("w"), "step {step} kept w");
            }
            let moved = driven.row(profile(), outline(3, StepArg::TargetY));
            assert_eq!(
                moved
                    .value
                    .map(|v| v.one().expect("one value").as_f64())
                    .ok(),
                Some(0.05)
            );
            assert!(
                !driven
                    .app
                    .drafts
                    .profile_edit
                    .as_ref()
                    .expect("held")
                    .differs_from_base()
            );
            // Drag w.
            let var = w(&driven);
            perform(&mut driven, SessionOp::BeginVariableGesture { var });
            perform(
                &mut driven,
                SessionOp::PreviewVariableGesture { var, value: 0.09 },
            );
            driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
            eprintln!("mid-drag drawn x = {}", drawn_x(&driven, 2));
            assert_eq!(drawn_x(&driven, 2), 0.09, "mid-drag the outline follows");
            perform(&mut driven, SessionOp::CommitVariableGesture { var });
            driven.quiet();
            eprintln!(
                "after release: drawn x = {}, held x = {}, committed reading = {:?}",
                drawn_x(&driven, 2),
                held_point(&driven, 2).x,
                driven
                    .row(profile(), outline(2, StepArg::TargetX))
                    .value
                    .map(|v| v.one().expect("one value").as_f64())
            );
            let painted = driven.quiet();
            eprintln!(
                "field runs after release: {:?}",
                painted
                    .iter()
                    .filter(|(r, _)| r.starts_with('='))
                    .map(|(r, _)| r.as_str())
                    .collect::<Vec<_>>()
            );
            let released = drawn_x(&driven, 2);
            perform(&mut driven, SessionOp::Undo);
            driven.quiet();
            eprintln!(
                "undo 1: drawn {} committed {:?}",
                drawn_x(&driven, 2),
                driven
                    .row(profile(), outline(2, StepArg::TargetX))
                    .value
                    .map(|v| v.one().expect("one value").as_f64())
            );
            perform(&mut driven, SessionOp::Undo);
            driven.quiet();
            eprintln!(
                "undo 2: far y {:?} src {:?} held far y {}",
                driven
                    .row(profile(), outline(3, StepArg::TargetY))
                    .value
                    .map(|v| v.one().expect("one value").as_f64()),
                driven.row(profile(), outline(2, StepArg::TargetX)).source,
                held_point(&driven, 3).y
            );
            // RED on 9afedc2593: the draft keeps the pre-drag reading.
            assert_eq!(released, 0.09, "after release the outline stays at w");
        }

        /// A value typed in the variable's row (one SetVariable) while the
        /// profile's draft is open and untouched.
        #[test]
        fn a_committed_variable_edit_moves_the_drawn_outline() {
            let mut driven = driven_plate();
            let var = w(&driven);
            perform(
                &mut driven,
                SessionOp::SetVariable {
                    var,
                    value: crate::props::SlotValue::Continuous(0.09),
                },
            );
            driven.quiet();
            eprintln!(
                "after SetVariable: drawn {} held {}",
                drawn_x(&driven, 2),
                held_point(&driven, 2).x
            );
            assert_eq!(drawn_x(&driven, 2), 0.09);
            perform(&mut driven, SessionOp::Undo);
            driven.quiet();
            eprintln!("after undo: drawn {}", drawn_x(&driven, 2));
            assert_eq!(drawn_x(&driven, 2), 0.07);
        }

        /// Decision 2: a draft holding edits keeps its preview over a
        /// variable drag.
        #[test]
        fn a_variable_drag_over_a_touched_draft() {
            let mut driven = driven_plate();
            touch_far_y(&mut driven, 0.05);
            let var = w(&driven);
            perform(&mut driven, SessionOp::BeginVariableGesture { var });
            perform(
                &mut driven,
                SessionOp::PreviewVariableGesture { var, value: 0.09 },
            );
            driven.frame(vec![egui::Event::PointerMoved(Driven::ELSEWHERE)]);
            eprintln!("touched mid-drag drawn x = {}", drawn_x(&driven, 2));
            assert_eq!(drawn_x(&driven, 2), 0.09, "the driven corner follows w");
        }

        /// Ops the doors under a held field emit, other than an
        /// expression, over held edits: are the edits kept?
        #[test]
        fn a_unit_pick_over_held_edits_keeps_them() {
            let mut driven = driven_plate();
            touch_far_y(&mut driven, 0.05);
            perform(
                &mut driven,
                SessionOp::SetSlotUnit {
                    node: profile(),
                    slot: outline(0, StepArg::PointX),
                    unit: pncad::quantity::MM.def(),
                },
            );
            driven.quiet();
            eprintln!("after unit pick: far y {}", held_point(&driven, 3).y);
            assert_eq!(held_point(&driven, 3).y, 0.05, "held edit kept");
        }

        #[test]
        fn a_rename_over_held_edits_keeps_them() {
            let mut driven = driven_plate();
            touch_far_y(&mut driven, 0.05);
            let var = w(&driven);
            perform(
                &mut driven,
                SessionOp::RenameVar {
                    var,
                    name: Some(VarName::from_static("width")),
                },
            );
            driven.quiet();
            eprintln!("after rename: far y {}", held_point(&driven, 3).y);
            assert_eq!(held_point(&driven, 3).y, 0.05, "held edit kept");
        }

        #[test]
        fn a_rename_then_apply_keeps_the_new_name() {
            let mut driven = driven_plate();
            let var = w(&driven);
            perform(
                &mut driven,
                SessionOp::RenameVar {
                    var,
                    name: Some(VarName::from_static("width")),
                },
            );
            driven.quiet();
            touch_far_y(&mut driven, 0.05);
            driven.click("Apply");
            driven.settle();
            let row = driven.row(profile(), outline(2, StepArg::TargetX));
            eprintln!("source after rename+apply: {:?}", row.source);
            assert_eq!(row.source.as_deref(), Some("width"));
        }

        #[test]
        fn a_deleted_variable_while_open() {
            let mut driven = driven_plate();
            touch_far_y(&mut driven, 0.05);
            let var = w(&driven);
            let out = driven.app.session.perform(SessionOp::DeleteVar { var });
            eprintln!("delete var refusal: {:?}", out.refusal);
            let painted = driven.quiet();
            let runs: Vec<&str> = painted.iter().map(|(r, _)| r.as_str()).collect();
            eprintln!(
                "draft held: {:?}; runs mention cannot: {:?}",
                driven
                    .app
                    .drafts
                    .profile_edit
                    .as_ref()
                    .map(|e| e.differs_from_base()),
                runs.iter()
                    .filter(|r| r.contains("cannot") || r.contains("resolve"))
                    .collect::<Vec<_>>()
            );
        }

        /// Decision 1, at the field: an expression typed over held edits.
        #[test]
        fn an_expression_typed_over_held_edits() {
            let mut driven = driven_plate();
            touch_far_y(&mut driven, 0.05);
            let before = driven.app.session.committed_doc().clone();
            let painted = driven.quiet();
            // The anchor's x field (first "x " name run).
            let at = painted
                .iter()
                .find(|(run, _)| run == "x ")
                .map(|(_, rect)| rect.center())
                .expect("a field named x");
            let press = |pressed| egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            driven.frame(vec![egui::Event::PointerMoved(at)]);
            driven.frame(vec![press(true), press(false)]);
            driven.frame(vec![egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            }]);
            driven.frame(vec![egui::Event::Text("w/7".to_owned())]);
            driven.frame(vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::default(),
            }]);
            let painted = driven.quiet();
            let runs: Vec<&str> = painted.iter().map(|(r, _)| r.as_str()).collect();
            let said = runs.iter().any(|r| r.contains("apply or revert"));
            eprintln!(
                "refusal said: {said}; doc unchanged: {}; far y {}; anchor {:?}",
                driven.app.session.committed_doc().bit_eq(&before),
                held_point(&driven, 3).y,
                held_point(&driven, 0)
            );
            assert!(said, "{runs:?}");
            assert!(driven.app.session.committed_doc().bit_eq(&before));
            assert_eq!(held_point(&driven, 3).y, 0.05);
        }
    }

    /// **The profile editor's fields are the profile's slots**: the
    /// startup plate's profile, opened in the editor from the Properties
    /// pane, offers each argument's expression, unit and range at the
    /// argument's own field.
    mod profile_editor_fields {
        use eframe::egui;
        use pncad::document::{RecipeNodeId, SlotId, StepArg};
        use pncad::profile::{Step, Target};

        use super::{Driven, startup_node};
        use crate::session::{BoundsTarget, Selection, SessionOp};

        /// The startup plate's profile.
        fn profile() -> RecipeNodeId {
            startup_node(0)
        }

        /// Argument `arg` of the outline's step `step`.
        fn outline(step: u32, arg: StepArg) -> SlotId {
            SlotId::Profile {
                loop_: 0,
                step,
                arg,
            }
        }

        /// Where the `nth` run `wanted` accepts was painted.
        fn nth(
            painted: &[(String, egui::Rect)],
            wanted: impl Fn(&str) -> bool,
            n: usize,
        ) -> egui::Pos2 {
            let hits: Vec<egui::Pos2> = painted
                .iter()
                .filter(|(run, _)| wanted(run))
                .map(|(_, rect)| rect.center())
                .collect();
            *hits
                .get(n)
                .unwrap_or_else(|| panic!("no run number {n} of {hits:?} in {painted:?}"))
        }

        /// A press at `at`, a pointer held down and moved `steps`
        /// frames rightward, `check` read while it is still down, then
        /// the release.
        fn drag(driven: &mut Driven, at: egui::Pos2, steps: usize, check: impl FnOnce(&Driven)) {
            let button = |pos, pressed| egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            driven.frame(vec![egui::Event::PointerMoved(at)]);
            driven.frame(vec![button(at, true)]);
            let mut to = at;
            for _ in 0..steps {
                to += egui::vec2(12.0, 0.0);
                driven.frame(vec![egui::Event::PointerMoved(to)]);
            }
            check(driven);
            driven.frame(vec![button(to, false)]);
            driven.frame(Vec::new());
        }

        /// The held draft's outline, step `step`.
        fn held_step(driven: &Driven, step: usize) -> Step<f64> {
            driven
                .app
                .drafts
                .profile_edit
                .as_ref()
                .expect("the editor holds the profile")
                .loops()[0][step]
        }

        /// **A driven argument opens in the editor, shows its
        /// expression, and its number is locked.** The outline's far
        /// corner's x read from a variable: the editor holds the
        /// profile rather than refusing it, says the expression at that
        /// argument, and a drag over its field moves neither the draft
        /// nor the document.
        #[test]
        fn a_driven_argument_opens_in_the_editor_with_its_number_locked() {
            let slot = outline(2, StepArg::TargetX);
            let mut driven = Driven::with(vec![
                SessionOp::DeclareVar {
                    name: pncad::document::VarName::from_static("w"),
                    value: pncad::document::FreeVar::continuous(
                        pncad::document::Dimension::Length,
                        0.07,
                    ),
                },
                SessionOp::SetSlotExpression {
                    node: profile(),
                    slot,
                    text: "w".to_owned(),
                },
                SessionOp::Select(Selection::Node(profile())),
            ]);
            let painted = driven.quiet();
            let runs: Vec<&str> = painted.iter().map(|(run, _)| run.as_str()).collect();
            assert!(
                !runs.iter().any(|run| run.contains("cannot hold")),
                "the editor holds a driven profile: {runs:?}"
            );
            assert!(
                runs.iter().any(|run| run.contains("= w")),
                "the driven argument says its expression: {runs:?}"
            );
            let before = held_step(&driven, 2);
            let Step::LineTo(Target::Point(corner)) = before else {
                panic!("the far corner is a point leg: {before:?}")
            };
            assert_eq!(
                corner.x, 0.07,
                "the held number is the expression's reading"
            );
            let committed = driven.app.session.committed_doc().clone();
            // The driven field is the one whose text is the driven
            // marker and its reading.
            let marker = format!("{} ", crate::props::DRIVEN);
            let at = nth(&painted, |run| run.starts_with(&marker), 0);
            drag(&mut driven, at, 6, |_| {});
            driven.settle();
            let Step::LineTo(Target::Point(after)) = held_step(&driven, 2) else {
                panic!("still a point leg")
            };
            assert_eq!(after.x, 0.07, "a drag over the driven field moves nothing");
            assert!(
                driven.app.session.committed_doc().bit_eq(&committed),
                "nor does it reach the document"
            );
        }

        /// **A live drag in the editor's field moves the drawn outline,
        /// and only the draft.** While the pointer is still down, the
        /// held anchor has moved, the loops the edit door previews are
        /// the moved ones, and the session holds no gesture: the
        /// document on screen is the committed one.
        #[test]
        fn a_drag_in_the_editor_moves_the_outline_live() {
            let mut driven = Driven::with(vec![SessionOp::Select(Selection::Node(profile()))]);
            let painted = driven.quiet();
            let before = held_step(&driven, 0);
            let Step::At(start) = before else {
                panic!("the outline opens with its anchor: {before:?}")
            };
            // A field's name is painted as its own run.
            let at = nth(&painted, |run| run == "x ", 0);
            drag(&mut driven, at, 6, |driven| {
                let Step::At(moved) = held_step(driven, 0) else {
                    panic!("still the anchor")
                };
                assert!(
                    moved.x > start.x,
                    "the held anchor follows the drag: {moved:?}"
                );
                let held = driven
                    .app
                    .drafts
                    .door_loops()
                    .edit
                    .expect("the edit door's loops");
                let crate::session::ProfileShape::Path { steps } = &held.loops[0] else {
                    panic!("a held loop is a path")
                };
                assert!(
                    matches!(steps[0], Step::At(p) if p.x == moved.x),
                    "what the viewport previews is the moved anchor"
                );
                assert!(
                    driven
                        .app
                        .session
                        .doc()
                        .bit_eq(driven.app.session.committed_doc()),
                    "no gesture holds a scratch document"
                );
            });
        }

        /// **The naming door waits on held edits**, as the expression
        /// door does: naming an argument's variable reloads the draft.
        /// Untouched, the anchor's row offers `name`; with another
        /// corner moved in the draft, no row does.
        #[test]
        fn the_naming_door_waits_on_held_edits() {
            let mut driven = Driven::with(vec![SessionOp::Select(Selection::Node(profile()))]);
            // The new-variable form, whose field also says `name`, is
            // behind its door, so every `name` counted is a row's door.
            let named = |painted: &[(String, egui::Rect)]| {
                painted.iter().filter(|(run, _)| run == "name").count()
            };
            let untouched = named(&driven.quiet());
            assert!(
                untouched > 0,
                "an untouched draft offers naming: {untouched}"
            );
            driven
                .app
                .drafts
                .profile_edit
                .as_mut()
                .expect("held")
                .steps_mut(0)[3] =
                Step::LineTo(Target::Point(pncad::geom_core::Point2::new(0.0, 0.05)));
            assert_eq!(
                named(&driven.quiet()),
                0,
                "held edits hold every row's naming door"
            );
        }

        /// **A unit change and a range probe made at the editor's field
        /// are the slot row's own operations.** The anchor's x is
        /// re-noted in millimetres first, so its point's one picker
        /// reads `mixed`; picking centimetres there writes both
        /// coordinates (`SetSlotUnit`), and the row's x range button
        /// asks the probe for that slot (`ProbeBounds`).
        #[test]
        fn a_unit_change_and_a_range_probe_at_the_field_reach_the_slot_ops() {
            let x = outline(0, StepArg::PointX);
            let y = outline(0, StepArg::PointY);
            let mut driven = Driven::with(vec![
                SessionOp::SetSlotUnit {
                    node: profile(),
                    slot: x,
                    unit: pncad::quantity::MM.def(),
                },
                SessionOp::Select(Selection::Node(profile())),
            ]);
            driven.quiet();
            driven.click("mixed");
            driven.click("cm");
            driven.settle();
            for slot in [x, y] {
                assert_eq!(
                    driven.row(profile(), slot).unit,
                    Some(pncad::quantity::CM.def()),
                    "{} is written in the unit picked",
                    slot.label()
                );
            }
            let painted = driven.quiet();
            // The row's range buttons are tagged by field: the first
            // `x` run that is a button alone is the anchor's.
            let at = nth(&painted, |run| run == "x", 0);
            driven.click_at(at);
            driven.settle();
            assert_eq!(
                driven
                    .app
                    .session
                    .bounds()
                    .map(|reading| reading.target.clone()),
                Some(BoundsTarget::Slot {
                    node: profile(),
                    slot: x,
                }),
                "the probe ran for the anchor's x"
            );
        }
    }

    /// **A real viewport click on a full-revolve rim, with the blend
    /// tool open, holds the whole circle** (review of PR 6129): the
    /// click goes through the frame's own `Tools::feed` call, so the
    /// `self.picks.index()` wiring is what this row reads. The startup
    /// plate is deleted first so nothing occludes the cylinder.
    #[test]
    fn a_viewport_click_on_a_seam_split_rim_holds_the_whole_circle() {
        use crate::session::EdgeSelection;
        use pncad::select::RoleSeg;
        let side = 0.01;
        let mut driven = Driven::with(vec![
            SessionOp::DeleteNode { node: placement() },
            SessionOp::DeleteNode { node: extrude() },
            SessionOp::DeleteNode {
                node: startup_node(0),
            },
            SessionOp::AddProfile {
                loops: vec![crate::test_support::rectangle_loop(
                    [0.0, 0.0],
                    side * 0.5,
                    side,
                )],
            },
        ]);
        let profile = *driven
            .app
            .session
            .committed_doc()
            .ids()
            .last()
            .expect("the profile was added");
        driven.perform(SessionOp::AddRevolve {
            profile,
            axis_through: [
                pncad::document::Formula::zero(pncad::document::Dimension::Length),
                pncad::document::Formula::zero(pncad::document::Dimension::Length),
            ],
            axis_heading: pncad::document::Formula::turn_over(4).expect("a quarter turn"),
            angle: crate::test_support::ang(core::f64::consts::TAU),
        });
        driven.settle();
        driven.app.tools.open(crate::tools::ToolKind::Blend);
        driven.quiet();

        let rect = driven
            .app
            .tree
            .tiles
            .iter()
            .find(|(_, tile)| matches!(tile, egui_tiles::Tile::Pane(super::Pane::Viewport)))
            .and_then(|(id, _)| driven.app.tree.tiles.rect(*id))
            .expect("the viewport is laid out");
        let ppp = f64::from(driven.ctx.pixels_per_point());
        let viewport = crate::input::ViewportSize {
            width_px: f64::from(rect.width()) * ppp,
            height_px: f64::from(rect.height()) * ppp,
        };
        let is_rim = |name: &pncad::prelude::StableName| {
            matches!(
                name.path.as_slice(),
                [.., RoleSeg::BandRim(_) | RoleSeg::BandRimPi(_)]
            )
        };
        let aimed = {
            let index = driven.on_screen();
            let (_, eval) = driven.app.session.landed_pair().expect("landed");
            let display = driven.app.session.display_view();
            let mut found = None;
            'scan: for i in 1..80 {
                for j in 1..50 {
                    let cursor = [
                        viewport.width_px * f64::from(i) / 80.0,
                        viewport.height_px * f64::from(j) / 50.0,
                    ];
                    if let Ok(Some(pick)) =
                        index.edge_at_for(eval, &driven.app.camera, viewport, cursor, &display)
                        && is_rim(&pick.name)
                        && pick.distance_px < 1.0
                    {
                        found = Some((cursor, pick.name.clone(), pick.node, pick.body));
                        break 'scan;
                    }
                }
            }
            found.expect("a rim is under some pixel of the viewport")
        };
        let (cursor, name, node, body) = aimed;
        let whole = driven
            .on_screen()
            .whole_edge(&EdgeSelection { name, node, body })
            .expect("the index answers the rim");
        assert_eq!(whole.len(), 2, "the circle under the cursor is two arcs");
        let at = egui::pos2(
            rect.min.x + (cursor[0] / ppp) as f32,
            rect.min.y + (cursor[1] / ppp) as f32,
        );
        driven.click_at(at);
        driven.quiet();
        let tool = driven.app.tools.blend().expect("the blend tool is open");
        let mut held = tool.selection();
        held.sort();
        let mut want = whole.clone();
        want.sort();
        assert_eq!(held, want, "one viewport click holds both arcs");
        assert_eq!(tool.count(), 1, "and counts one pick");
    }
}

/// **A click in the viewport while the index seam is busy**, driven
/// through the whole app: the value `ViewerApp::ui` hands the viewport
/// ([`ViewerBehavior::indexing`]) is what decides which refusal the
/// click earns, and this row is where that call site is read.
#[cfg(test)]
mod index_seam_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use eframe::egui;
    use egui_tiles::Tile;

    use super::properties_pane_tests::{WINDOW, app_frame, settle};
    use super::{Pane, ViewerApp};
    use crate::evalseam::{FitDone, FitRequest, FitService};
    use crate::frame::Progress;
    use crate::pickcache::NotIndexed;
    use crate::session::DocSession;

    /// A fit seam that is always pricing and never answers: the
    /// window between a document landing and its δ being chosen, held
    /// open for as many frames as the row needs.
    struct Pricing;

    impl FitService for Pricing {
        fn submit(&mut self, _request: FitRequest) {}
        fn poll(&mut self) -> Option<FitDone> {
            None
        }
        fn busy(&self) -> bool {
            true
        }
    }

    /// **The click is told an index is coming**, over a document that
    /// has landed while its δ is still being fitted. That is the state
    /// a document opens into: `sync_scene` hands the pick cache no δ,
    /// so the cache holds no attempt and its own record reads idle, and
    /// only a viewport handed the index seam's door hears the fit.
    #[test]
    fn a_click_while_the_fit_prices_the_delta_is_told_an_index_is_coming() {
        let ctx = egui::Context::default();
        let tol = pncad::tolerance::witness();
        let mut app =
            ViewerApp::assemble(&ctx, tol).expect("startup that needs no graphics device");
        let (document, _) = crate::scene::plate_with_hole(tol).expect("the startup document");
        let mut session = DocSession::inline(document, tol);
        session.pump();
        app.session = session;
        app.fit = Box::new(Pricing);
        let mut frame = eframe::Frame::_new_kittest();
        let mut time = 0.0;
        let mut paint = |app: &mut ViewerApp, events: Vec<egui::Event>| {
            time += 1.0;
            app_frame(&ctx, app, &mut frame, WINDOW, Some(time), events)
                .into_iter()
                .map(|landed| landed.text)
                .collect::<Vec<_>>()
        };

        // The fit never answers, so the app rests indexing rather than
        // quiescent.
        settle(
            &mut app,
            |app| app.progress() == Some(Progress::Indexing),
            |app| paint(app, Vec::new()),
        );
        assert!(
            !app.picks.indexing() && app.picks.index().is_none(),
            "the fixture is the cache's idle record under a fit in flight"
        );
        let at = app
            .tree
            .tiles
            .iter()
            .find(|(_, tile)| matches!(tile, Tile::Pane(Pane::Viewport)))
            .and_then(|(id, _)| app.tree.tiles.rect(*id))
            .expect("the viewport is laid out")
            .center();
        let press = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        paint(&mut app, vec![egui::Event::PointerMoved(at)]);
        paint(&mut app, vec![press(true), press(false)]);
        // Two frames: the status line's scroll area is laid out on the
        // first frame it appears in and painted on the next.
        paint(&mut app, Vec::new());
        let painted = paint(&mut app, Vec::new());

        assert!(
            painted.contains(&NotIndexed::Building.to_string()),
            "the click's refusal promises the index the toolbar is spinning for: {painted:?}"
        );
        assert!(
            !painted.contains(&NotIndexed::Absent.to_string()),
            "and does not say none is being built"
        );
    }
}
