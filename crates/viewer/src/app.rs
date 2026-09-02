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
use pncad::document::{
    Axis3, BooleanOp, Dimension, DimensionError, DocumentId, Expr, LoopProgram, ParamName,
    ProductError, RecipeNodeId, SlotId,
};
use pncad::geom_core::Tol;
use pncad::profile::{ArcSide, ArcSweep};
use pncad::quantity::{self, AngleUnit, LengthUnit, UnitDef, WrittenAngle, WrittenLength};

use crate::blend::{BlendError, BlendKindChoice, BlendTarget, FREEZE_NOTE};
use crate::camera::{self, Camera, CameraOp};
use crate::datums;
use crate::display::{DisplayView, free_move_check};
use crate::evalseam::Generation;
#[cfg(not(target_family = "wasm"))]
use crate::evalseam::ThreadEvaluator;
use crate::frame::{self, IdQueryLog, IdStep, StatusUpdate};
use crate::gpu::{DEPTH_BITS, IdQuery, ViewportCallback, ViewportRenderer};
use crate::input::{self, InputMap, PointerButton, ViewportEvent, ViewportSize};
use crate::matetool::{MateChoice, MateToolState, admitted_classes};
use crate::parts::PartChooser;
use crate::pick::{self, PickCache, PickIndex};
use crate::prefs::{self, Prefs, PrefsStore};
use crate::props::{self, SlotDriver, SlotGroup, SlotRow, SlotValue};
use crate::scene::{self, DisplayTolerance, SceneMesh};
use crate::seats::{Seat, SeatError, seat_line};
use crate::session::{
    BoundsTarget, DatumSpec, DocSession, ProfileShape, Refusal, Selection, SessionOp, Standing,
};
use crate::sketch::{self, ArcSpec, PathStep, PathTarget, PreviewError, ProfilePreview};
use crate::theme::{Polarity, Theme};
use crate::tools::{ToolKind, Tools};
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
const GLYPH_REMOVE: &str = "×";
/// Move a list row earlier — see [`GLYPH_REMOVE`] for the font rule.
const GLYPH_UP: &str = "⬆";
/// Move a list row later — see [`GLYPH_REMOVE`] for the font rule.
const GLYPH_DOWN: &str = "⬇";
/// Marks a product root in the feature tree — see [`GLYPH_REMOVE`].
const GLYPH_ROOT: &str = "»";

/// **The picture's own size**, in metres: the diagonal of the scene's
/// bounding box, which is what a datum is drawn relative to.
///
/// Zero for a scene with no geometry — an emptied document, or one
/// holding only datums — and `datums::draws` turns that into its own
/// fallback rather than this function inventing one. Two places would
/// be two answers to "how big is nothing".
fn scene_extent(scene: &SceneMesh) -> f64 {
    let bounds = scene.bounds();
    let span = |axis| bounds.max(axis) - bounds.min(axis);
    let (x, y, z) = (span(bvh::Axis::X), span(bvh::Axis::Y), span(bvh::Axis::Z));
    (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

/// **How big the tip marks in a profile preview are**, in sketch-plane
/// metres: a fraction of the whole preview's extent.
///
/// Relative rather than absolute because a preview has no fixed scale
/// — a 2 mm boss and a 2 m plate go through this same form — and
/// relative to the WHOLE preview rather than to each loop, so a bore's
/// marks match its outer's. A preview with no extent at all (a single
/// authored point, nothing yet) has nothing to take a fraction of and
/// gets no marks; a cross of size zero would be no mark anyway.
fn tip_mark(loops: &[sketch::PreviewLoop]) -> f64 {
    let points = loops.iter().flat_map(|drawn| drawn.points.iter());
    let mut lo = [f64::INFINITY; 2];
    let mut hi = [f64::NEG_INFINITY; 2];
    for point in points {
        for axis in 0..2 {
            lo[axis] = lo[axis].min(point[axis]);
            hi[axis] = hi[axis].max(point[axis]);
        }
    }
    let diagonal = (hi[0] - lo[0]).hypot(hi[1] - lo[1]);
    if diagonal.is_finite() && diagonal > 0.0 {
        diagonal * TIP_MARK_FRACTION
    } else {
        0.0
    }
}

/// The share of a preview's diagonal one tip mark spans — small enough
/// that a dense chain does not become a field of crosses, large enough
/// to read against the geometry it sits on. The heading tick is twice
/// this again, because a direction has to be long enough to have one.
///
/// Set by looking: at 0.025 it was under a pixel on a profile filling
/// a third of the viewport, which is a mark nobody can see — and a
/// sketch plane seen at a grazing angle foreshortens whatever is left.
const TIP_MARK_FRACTION: f64 = 0.07;

/// **Which way the chain leaves the vertex at `at`** — a unit vector,
/// or `None` where there is no next point to take one from.
///
/// The next flattened point, which is the tangent to within the chord
/// tolerance the preview was flattened at. At the LAST vertex of an
/// open chain there is no leaving direction, so the INCOMING one is
/// answered instead: that tip is where the chain currently ends, and
/// the heading a reader wants there is the one it arrived on.
fn heading(points: &[[f64; 2]], at: usize, closed: bool) -> Option<[f64; 2]> {
    let (from, to) = if at + 1 < points.len() {
        (points[at], points[at + 1])
    } else if closed && points.len() > 1 {
        (points[at], points[0])
    } else if at > 0 {
        (points[at - 1], points[at])
    } else {
        return None;
    };
    let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
    let length = dx.hypot(dy);
    (length > 0.0).then(|| [dx / length, dy / length])
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
    /// **The unit every creation form's LENGTH field is written in.**
    ///
    /// ONE choice for all the forms, not one per form. The panel's
    /// pickers are per literal because a literal is a thing in a
    /// document that remembers its own notation; a form's is a
    /// statement about how the person at the keyboard is working, and
    /// somebody who authors a datum in millimetres is not then
    /// authoring the extrude that consumes it in metres. The drafts
    /// behind the fields stay canonical either way ([`unit_field`]),
    /// so moving the picker re-writes what is on screen and changes
    /// no value.
    /// Not optional: an authored value always names the notation it is
    /// written in (`quantity::written`'s module docs), so the field and
    /// the picker beside it read one fact rather than each resolving an
    /// absence its own way.
    length_unit: LengthUnit,
    /// The same for every ANGLE field. Defaults to half turns
    /// (`pi rad`), the notation this editor says angles in.
    angle_unit: AngleUnit,
    /// The add-profile form's shape choice — `None` until the user
    /// makes one.
    ///
    /// **Optional because the form is LIVE.** Every other draft here
    /// is a number sitting in a field, but this one decides what the
    /// viewport draws: the add-profile preview is taken from these
    /// drafts each frame, so a pre-selected shape meant that merely
    /// opening "Add feature" — to reach the datum form, say — put a
    /// circle in the picture that nobody had asked for. `None` is the
    /// form at rest: no shape fields, no preview, and the commit
    /// disabled until a shape is chosen.
    profile_shape: Option<ShapeKind>,
    /// The path form's verbs, in authoring order.
    ///
    /// Starts as the SQUARE a `line_to` chain spells — an empty list
    /// is a form with nothing to look at and no example of what a
    /// chain is supposed to look like, and this one is four verbs a
    /// reader can take apart. It is a draft like every other field
    /// here: nothing reaches a document until Add profile.
    profile_path: Vec<PathStep>,
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
    /// The boolean tool's operation choice.
    boolean_op: BooleanOp,
    /// The transform tool's translation, metres.
    transform_translation: [f64; 3],
    /// Its rotation axis (unitless).
    transform_axis: [f64; 3],
    /// Its rotation angle, radians.
    transform_angle: f64,
    /// The pattern tool's rule choice.
    pattern_kind: PatternKindChoice,
    /// Its instance count — an INTEGER all the way from the field,
    /// because the slot it lands in is Count-typed and a number that
    /// was rounded on the way could differ from the one on screen.
    pattern_count: i64,
    /// The linear rule's step direction (unitless).
    pattern_direction: [f64; 3],
    /// The linear rule's spacing, metres.
    pattern_spacing: f64,
    /// The circular rule's angular step, radians.
    pattern_step: f64,
    /// The blend form's kind choice — which of the two doors the
    /// commit button calls.
    blend_kind: BlendKindChoice,
    /// Its one Length field, metres. ONE field for both kinds by the
    /// unit's spec: what the number means is the kind's to say
    /// ([`BlendKindChoice::size_label`]), and a second field would be
    /// a second place for the same quantity to be typed into.
    blend_size: f64,
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
            length_unit: quantity::M,
            angle_unit: quantity::PI,
            profile_shape: None,
            profile_path: vec![
                PathStep::At([0.0, 0.0]),
                PathStep::LineTo(PathTarget::Point([0.01, 0.0])),
                PathStep::LineTo(PathTarget::Point([0.01, 0.01])),
                PathStep::LineTo(PathTarget::Point([0.0, 0.01])),
                PathStep::LineTo(PathTarget::Start),
            ],
            profile_centre: [0.0; 2],
            profile_radius: 0.01,
            profile_bored: false,
            profile_bore: 0.005,
            profile_extent: [0.01, 0.01],
            extrude_distance: 0.01,
            revolve_angle: core::f64::consts::TAU,
            boolean_op: BooleanOp::Union,
            transform_translation: [0.0; 3],
            transform_axis: [0.0, 0.0, 1.0],
            transform_angle: 0.0,
            pattern_kind: PatternKindChoice::Linear,
            pattern_count: 3,
            pattern_direction: [1.0, 0.0, 0.0],
            pattern_spacing: 0.02,
            pattern_step: core::f64::consts::FRAC_PI_2,
            blend_kind: BlendKindChoice::Fillet,
            blend_size: 0.001,
        }
    }
}

impl Drafts {
    /// **The loops the add-profile form would author right now.**
    ///
    /// One home, read twice: by the form's commit button and by the
    /// preview drawn under and around it. Two readings that built the
    /// loops separately could draw one shape and author another,
    /// which is the one way a live preview can lie.
    ///
    /// The bore is a second loop rather than a field on the first —
    /// which is what makes the ring demo's annulus a template rather
    /// than a special case — and the form's own guard on it lives at
    /// the commit ([`ViewerBehavior::add_profile_ui`]), because it is
    /// about what the loops MEAN, not about what they are.
    /// No shape chosen yet is the empty list, not a refusal: the form
    /// authors nothing and the preview draws nothing, which is what
    /// [`Drafts::profile_shape`] being `None` means.
    fn profile_loops(&self) -> Vec<ProfileShape> {
        match self.profile_shape {
            None => Vec::new(),
            Some(ShapeKind::Circle) => {
                let mut loops = vec![ProfileShape::Circle {
                    centre: self.profile_centre,
                    radius: self.profile_radius,
                }];
                if self.profile_bored {
                    loops.push(ProfileShape::Circle {
                        centre: self.profile_centre,
                        radius: self.profile_bore,
                    });
                }
                loops
            }
            Some(ShapeKind::Rectangle) => vec![ProfileShape::Rectangle {
                width: self.profile_extent[0],
                height: self.profile_extent[1],
            }],
            Some(ShapeKind::Path) => vec![ProfileShape::Path {
                steps: self.profile_path.clone(),
            }],
        }
    }

    /// **The notation these forms are authoring in** — the two pickers,
    /// as the lowering wants them.
    fn notation(&self) -> sketch::Notation {
        sketch::Notation {
            length: self.length_unit,
            angle: self.angle_unit,
        }
    }

    /// The loop PROGRAMS the add-profile form would author right now:
    /// [`Drafts::profile_loops`] lowered in this form's notation, which
    /// is what the op carries.
    ///
    /// # Errors
    ///
    /// A non-finite field (the literal door's refusal).
    fn profile_programs(&self) -> Result<Vec<LoopProgram>, DimensionError> {
        self.profile_loops()
            .iter()
            .map(|shape| sketch::loop_program(shape, self.notation()))
            .collect()
    }

    /// A `Length` literal from a draft field, remembering the form's
    /// notation. The draft is already canonical — a picker re-writes
    /// what is on screen and changes no value — so this attaches the
    /// unit without applying it.
    ///
    /// # Errors
    ///
    /// A non-finite draft (the literal door's refusal).
    fn length(&self, metres: f64) -> Result<Expr, DimensionError> {
        Expr::written_length(WrittenLength::canonical_in(metres, self.length_unit))
    }

    /// An `Angle` literal from a draft field — [`Drafts::length`]'s
    /// twin.
    ///
    /// # Errors
    ///
    /// A non-finite draft.
    fn angle(&self, radians: f64) -> Result<Expr, DimensionError> {
        Expr::written_angle(WrittenAngle::canonical_in(radians, self.angle_unit))
    }

    /// Three `Length` literals — a datum origin, a translation.
    ///
    /// # Errors
    ///
    /// A non-finite component.
    fn lengths(&self, v: [f64; 3]) -> Result<[Expr; 3], DimensionError> {
        Ok([self.length(v[0])?, self.length(v[1])?, self.length(v[2])?])
    }
}

/// Three dimensionless literals — a normal, a direction, a rotation
/// axis. Not a [`Drafts`] method, because there is no notation to
/// carry from the form: a dimensionless number has one spelling, and
/// `Expr::literal` stores that row itself.
///
/// # Errors
///
/// A non-finite component.
fn scalars(v: [f64; 3]) -> Result<[Expr; 3], DimensionError> {
    Ok([
        Expr::literal(v[0], Dimension::Scalar)?,
        Expr::literal(v[1], Dimension::Scalar)?,
        Expr::literal(v[2], Dimension::Scalar)?,
    ])
}

/// Why a creation form's commit did not produce an op.
///
/// Two faults reach one button: a seat a tool still needs, and a draft
/// the literal door refuses. They are separate types because they are
/// separate facts — one is about the picks, one about the numbers — and
/// this carries them to the status line without flattening either into
/// a string at the raising site.
#[derive(Debug)]
enum CommitFault {
    /// A tool seat is still empty.
    Seat(SeatError),
    /// A draft field is not a value a literal may hold.
    Dimension(DimensionError),
}

impl From<SeatError> for CommitFault {
    fn from(error: SeatError) -> Self {
        Self::Seat(error)
    }
}

impl From<DimensionError> for CommitFault {
    fn from(error: DimensionError) -> Self {
        Self::Dimension(error)
    }
}

impl std::fmt::Display for CommitFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seat(error) => error.fmt(f),
            Self::Dimension(error) => error.fmt(f),
        }
    }
}

/// The pattern form's rule choice — the two PARAMETRIC rules, an enum
/// for the reason [`DatumKind`] is one. `Explicit` is absent by the
/// plan's ruling: a list of absolute frames is not a form's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternKindChoice {
    /// Stepped along a direction.
    Linear,
    /// Stepped around a picked datum axis.
    Circular,
}

impl PatternKindChoice {
    /// Every rule with its radio label, in form order.
    const ALL: [(Self, &'static str); 2] = [(Self::Linear, "linear"), (Self::Circular, "circular")];
}

/// The boolean operations the form offers, with their labels — the
/// KERNEL's enum and its own words, so the button a user reads and the
/// operation the node carries cannot drift into two vocabularies.
const BOOLEAN_OPS: [(BooleanOp, &str); 3] = [
    (BooleanOp::Union, "union"),
    (BooleanOp::Subtract, "subtract"),
    (BooleanOp::Intersect, "intersect"),
];

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

/// The add-profile form's loop choice: the two templates, or a PATH
/// authored verb by verb.
///
/// An enum for the reason [`DatumKind`] is one — and the templates
/// stay in it rather than being folded into the path arm because they
/// are not chains: a circle is a seamless closed carrier no chain of
/// legs can spell, and a rectangle is four `line_to`s nobody should
/// have to type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShapeKind {
    /// A circle, optionally with a concentric bore.
    Circle,
    /// A centred rectangle.
    Rectangle,
    /// A chain of authoring verbs — the whole PATHS vocabulary.
    Path,
}

impl ShapeKind {
    /// Every shape with its radio label, in form order.
    const ALL: [(Self, &'static str); 3] = [
        (Self::Circle, "circle"),
        (Self::Rectangle, "rectangle"),
        (Self::Path, "path"),
    ];
}

/// **The authoring verbs the path form offers**, with the names the
/// algebra itself gives them.
///
/// A tag beside [`PathStep`] rather than a method on it: the form
/// needs to name a verb BEFORE it has a step (the "add" control's
/// choice), and a step needs to name its own verb (the row's combo),
/// so the tag is the thing both hold. [`PathVerb::fresh`] is the one
/// place a default step per verb is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathVerb {
    /// Bind the tip's position.
    At,
    /// Bind the tip's outgoing direction, absolutely.
    Angle,
    /// Bind it by exact components.
    Toward,
    /// Leave along the incoming tangent.
    Tangent,
    /// Leave along its reverse.
    Cusp,
    /// Leave at an angle from it.
    Turn,
    /// A straight leg of a stated length.
    Line,
    /// A straight leg to a target.
    LineTo,
    /// A sharp arc leg.
    ArcTo,
    /// An arc leg leaving along the bound direction.
    TangentArcTo,
    /// A structural vertex on the incoming carrier.
    ArcContinue,
    /// Round the corner: line in, line out.
    Fillet,
    /// Round it with an arc on the arrival side.
    FilletArc,
    /// Round it with an arc on the incoming side.
    ArcFillet,
    /// Round it with an arc on both.
    ArcFilletArc,
    /// The anchor a fillet's arrival side is aimed at.
    FarEndTo,
    /// The seam fillet's close.
    CloseTo,
}

impl PathVerb {
    /// Every verb, in the algebra's own order — the "add step" menu
    /// and the row combo's options. Labels come from
    /// [`PathVerb::label`], so this list carries the ORDER and
    /// nothing a second copy of it could get wrong.
    const ALL: [Self; 17] = [
        Self::At,
        Self::Angle,
        Self::Toward,
        Self::Tangent,
        Self::Cusp,
        Self::Turn,
        Self::Line,
        Self::LineTo,
        Self::ArcTo,
        Self::TangentArcTo,
        Self::ArcContinue,
        Self::Fillet,
        Self::FilletArc,
        Self::ArcFillet,
        Self::ArcFilletArc,
        Self::FarEndTo,
        Self::CloseTo,
    ];

    /// This verb's label — a match rather than a search through
    /// [`PathVerb::ALL`], so a verb with no label is a compile error
    /// rather than a `?` on somebody's screen. (Whether a verb
    /// reaches the MENU is [`PathVerb::ALL`]'s to answer, and nothing
    /// checks that: the type is private behind the `app` feature, so
    /// no row can see it — issue #1385.)
    fn label(self) -> &'static str {
        match self {
            Self::At => "at",
            Self::Angle => "angle",
            Self::Toward => "toward",
            Self::Tangent => "tangent",
            Self::Cusp => "cusp",
            Self::Turn => "turn",
            Self::Line => "line",
            Self::LineTo => "line_to",
            Self::ArcTo => "arc_to",
            Self::TangentArcTo => "tangent_arc_to",
            Self::ArcContinue => "arc_continue",
            Self::Fillet => "fillet",
            Self::FilletArc => "fillet_arc",
            Self::ArcFillet => "arc_fillet",
            Self::ArcFilletArc => "arc_fillet_arc",
            Self::FarEndTo => "to (far end)",
            Self::CloseTo => "to Start (close)",
        }
    }

    /// Which verb a step names.
    fn of(step: &PathStep) -> Self {
        match step {
            PathStep::At(_) => Self::At,
            PathStep::Angle(_) => Self::Angle,
            PathStep::Toward { .. } => Self::Toward,
            PathStep::Tangent => Self::Tangent,
            PathStep::Cusp => Self::Cusp,
            PathStep::Turn(_) => Self::Turn,
            PathStep::Line(_) => Self::Line,
            PathStep::LineTo(_) => Self::LineTo,
            PathStep::ArcTo(_) => Self::ArcTo,
            PathStep::TangentArcTo(_) => Self::TangentArcTo,
            PathStep::ArcContinue(_) => Self::ArcContinue,
            PathStep::Fillet(_) => Self::Fillet,
            PathStep::FilletArc { .. } => Self::FilletArc,
            PathStep::ArcFillet { .. } => Self::ArcFillet,
            PathStep::ArcFilletArc { .. } => Self::ArcFilletArc,
            PathStep::FarEndTo(_) => Self::FarEndTo,
            PathStep::CloseTo => Self::CloseTo,
        }
    }

    /// A step of this verb with the form's starting numbers.
    ///
    /// **Millimetre-scale, never zero.** A leg of length zero and a
    /// fillet of radius zero are both geometry refusals, so a fresh
    /// step that carried them would put the form in a refusing state
    /// the moment a verb was added — which reads as the form
    /// rejecting the verb rather than waiting for its number.
    fn fresh(self) -> PathStep {
        let point = [0.01, 0.0];
        let arc = ArcSpec::Radius {
            r: 0.01,
            side: ArcSide::Left,
        };
        match self {
            Self::At => PathStep::At([0.0, 0.0]),
            Self::Angle => PathStep::Angle(0.0),
            Self::Toward => PathStep::Toward { dx: 1.0, dy: 0.0 },
            Self::Tangent => PathStep::Tangent,
            Self::Cusp => PathStep::Cusp,
            Self::Turn => PathStep::Turn(0.0),
            Self::Line => PathStep::Line(0.01),
            Self::LineTo => PathStep::LineTo(PathTarget::Point(point)),
            Self::ArcTo => PathStep::ArcTo(arc),
            Self::TangentArcTo => PathStep::TangentArcTo(PathTarget::Point(point)),
            Self::ArcContinue => PathStep::ArcContinue(point),
            Self::Fillet => PathStep::Fillet(0.001),
            Self::FilletArc => PathStep::FilletArc {
                radius: 0.001,
                spec: arc,
            },
            Self::ArcFillet => PathStep::ArcFillet {
                spec: arc,
                radius: 0.001,
            },
            Self::ArcFilletArc => PathStep::ArcFilletArc {
                spec: arc,
                radius: 0.001,
                spec2: arc,
            },
            Self::FarEndTo => PathStep::FarEndTo(point),
            Self::CloseTo => PathStep::CloseTo,
        }
    }
}

/// **Which of [`ArcSpec`]'s six modes the form is offering** — the
/// tag [`PathVerb`] is, for the reason it is one: the picker needs to
/// name a mode before there is a spec in it, and a spec needs to name
/// its own mode. An index into a label table would couple the two by
/// position, so a reordered table would silently relabel every mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArcMode {
    /// The carrier's radius and the side its centre is on.
    Radius,
    /// The endpoint and an authored bulge.
    Bulge,
    /// A point the arc passes through, and the endpoint.
    Via,
    /// The carrier centre, the travel sense, and the endpoint.
    Center,
    /// The carrier and how far round it to go.
    Sweep,
    /// The carrier and the distance travelled along it.
    ArcLen,
}

impl ArcMode {
    /// Every mode, in the vocabulary's own order — the picker's
    /// options.
    const ALL: [Self; 6] = [
        Self::Radius,
        Self::Bulge,
        Self::Via,
        Self::Center,
        Self::Sweep,
        Self::ArcLen,
    ];

    /// This mode's label.
    fn label(self) -> &'static str {
        match self {
            Self::Radius => "radius",
            Self::Bulge => "bulge",
            Self::Via => "via",
            Self::Center => "centre",
            Self::Sweep => "sweep",
            Self::ArcLen => "arc length",
        }
    }

    /// Which mode a spec is in.
    fn of(spec: &ArcSpec) -> Self {
        match spec {
            ArcSpec::Radius { .. } => Self::Radius,
            ArcSpec::Bulge { .. } => Self::Bulge,
            ArcSpec::Via { .. } => Self::Via,
            ArcSpec::Center { .. } => Self::Center,
            ArcSpec::Sweep { .. } => Self::Sweep,
            ArcSpec::ArcLen { .. } => Self::ArcLen,
        }
    }

    /// A spec of this mode with the form's starting numbers —
    /// millimetre-scale and never degenerate, for the reason
    /// [`PathVerb::fresh`]'s are.
    fn fresh(self) -> ArcSpec {
        let target = PathTarget::Point([0.01, 0.0]);
        match self {
            Self::Radius => ArcSpec::Radius {
                r: 0.01,
                side: ArcSide::Left,
            },
            Self::Bulge => ArcSpec::Bulge { target, b: 0.5 },
            Self::Via => ArcSpec::Via {
                q: [0.005, 0.005],
                target,
            },
            Self::Center => ArcSpec::Center {
                c: [0.0, 0.0],
                winding: ArcSweep::Ccw,
                target,
            },
            Self::Sweep => ArcSpec::Sweep {
                r: 0.01,
                side: ArcSide::Left,
                angle: core::f64::consts::FRAC_PI_2,
            },
            Self::ArcLen => ArcSpec::ArcLen {
                r: 0.01,
                side: ArcSide::Left,
                len: 0.01,
            },
        }
    }
}

/// One drag tick of a LENGTH field, in metres — half a millimetre.
/// The creation forms' and the property panel's alike ([`drag_tick`]
/// is where the panel picks it), so one gesture over a length cannot
/// come to mean two different steps.
const FIELD_DRAG_SPEED: f64 = 0.0005;

/// One drag tick of an ANGLE field, in radians — a third of a degree,
/// so a full turn is a drag of a few hundred pixels rather than of
/// several screens.
///
/// A separate constant because the unit is: dragging a radian field at
/// the metre field's speed moves it by 0.0005 rad per pixel, which is
/// a quarter-turn per three thousand pixels.
const ANGLE_DRAG_SPEED: f64 = 0.005;

/// One drag tick of a DIMENSIONLESS field — a direction or a normal
/// component, whose useful range is roughly [-1, 1].
///
/// The length speed applied here made these fields effectively
/// undraggable: at 0.0005 per pixel, moving a component from 0 to 1
/// took two thousand pixels of drag. A hundredth per pixel spans the
/// whole range in one comfortable gesture, and the exact value stays a
/// keyboard edit either way.
const UNIT_DRAG_SPEED: f64 = 0.01;

/// One drag tick of a COUNT field — instances are whole, so the field
/// is dragged in tenths of one and lands on integers.
const COUNT_DRAG_SPEED: f64 = 0.1;

/// **The drag tick a slot of `dimension` is scrubbed at**, in
/// CANONICAL units — the property panel's pick from the same three
/// constants the creation forms choose between by hand.
///
/// A dimension branch and not one number, because the useful range of
/// a slot is its dimension's: half a millimetre per pixel is a good
/// length tick and a terrible angle one — at 0.0005 rad it takes
/// twelve thousand pixels to drag a full turn, which is the same
/// arithmetic [`ANGLE_DRAG_SPEED`] exists to answer for the forms.
/// A `Count` never reaches here (its slots are structural, and the
/// panel steps those in whole units), so it takes the count tick for
/// completeness rather than for use.
fn drag_tick(dimension: Dimension) -> f64 {
    match dimension {
        Dimension::Length => FIELD_DRAG_SPEED,
        Dimension::Angle => ANGLE_DRAG_SPEED,
        Dimension::Scalar => UNIT_DRAG_SPEED,
        Dimension::Count => COUNT_DRAG_SPEED,
    }
}

/// **The smallest pattern count the form offers.**
///
/// A pattern of zero instances refuses at evaluation
/// (`NonPositiveCount`, typed, on the node's own badge), so the form
/// declines to author one — the same rule the bore field follows. There
/// is deliberately NO upper bound: the property panel imposes none on
/// the slot afterwards, and a cap here would be a limit the document
/// does not have.
const MIN_PATTERN_COUNT: i64 = 1;

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
    /// **What the open tool said about THIS frame** — declined picks
    /// and survival drops, collected as they happen and applied with
    /// the batch verdict rather than before it.
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
                    // **A document with no body root is EMPTY, not
                    // malformed.** A fresh document is in that state,
                    // and so is one whose last feature was just
                    // deleted — and the blank viewport this arm has
                    // just drawn says so more plainly than a line of
                    // text could. Reporting it made deleting the last
                    // feature look like a failure. Every other gather
                    // refusal is a fault no tree badge carries, which
                    // is what this line exists for, and stays here.
                    .filter(|fault| !matches!(fault, ProductError::NoBodyRoots))
                    // `ProductError`'s own `Display` opens every arm
                    // with "product: ", so the prefix this used to add
                    // by hand said the word twice.
                    .map(ToString::to_string);
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
        let notices = core::mem::take(&mut self.notices);
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

impl ViewerApp {
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
        let profile_preview = self.profile_form_drawn.then(|| {
            sketch::preview(
                sketch::form_plane(),
                &authored,
                self.session.tol(),
                self.delta.get(),
            )
        });
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
    /// The modal tools, at most one open.
    tools: &'a mut Tools,
    /// The `Add part…` chooser, if open.
    part_chooser: &'a mut Option<PartChooser>,
    /// What the add-profile form's loops would draw, taken once for
    /// the frame: the panel says what it refuses and the viewport
    /// draws what it replayed, from ONE reading.
    ///
    /// `None` is "no preview was taken this frame" — the frame the
    /// form first comes on screen, before the latch below has told
    /// anyone to take one. Distinct from `Some(Ok(empty))`, which is
    /// a preview that WAS taken and drew nothing, and which the form
    /// is entitled to say so about.
    profile_preview: &'a Option<Result<ProfilePreview, PreviewError>>,
    /// Set by the add-profile form while it draws; read next frame.
    profile_form_drawn: &'a mut bool,
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
    /// Whether the viewport draws datums ([`ViewerApp::show_datums`]);
    /// the View pane's checkbox writes through it.
    show_datums: &'a mut bool,
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
        // re-decide it** — which tool narrows what is
        // `ToolKind::pick_kinds`, an exhaustive match beside the tool
        // vocabulary, and the narrowing travels through `hovered_for`,
        // the one door that answers what a cursor means, so a tool
        // cannot end up on a different rule.
        let kinds = self.tools.pick_kinds();
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
        let mut edges = self
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
        // **The open blend tool's held set is marked too** — all of
        // it, because the set IS what the user is composing and a
        // count alone cannot tell them WHICH twelve edges they hold.
        //
        // Marked as SELECTED, the mark meaning "a choice you have
        // made". `BlendTool::mark_segments` applies the same (node,
        // body) narrowing a single selection gets — one pass over the
        // target's drawn edges, so the cost is the body's edge count
        // and not its square.
        if let (Some(index), Some(tool)) = (self.index, self.tools.blend()) {
            edges
                .selected
                .extend(tool.mark_segments(index, self.display));
        }
        // **The profile being authored, drawn where it would land.**
        //
        // The form's loops are on a sketch plane, so they HAVE a
        // place: the wireframe goes in the viewport, at that place,
        // rather than into a pane of its own — a preview beside the
        // model cannot show what a preview is mostly for, which is
        // whether the shape is the right size and in the right spot
        // relative to what is already there.
        //
        // Drawn in the probe mark, never the selection mark, because
        // it is not in the document (`EdgeOverlay::preview`). A
        // preview that failed to replay draws nothing and says why in
        // the form; one that replayed but does not VALIDATE draws
        // anyway, which is the case where looking at it is the whole
        // point.
        // **The document's construction geometry**, drawn before the
        // preview so a form composing something over a datum reads on
        // top of it. Sized against the scene's own extent
        // (`datums::draws`), so a datum is the same size relative to
        // the part whatever the part's scale is.
        if let Some((doc, evaluation)) = self.session.landed_pair().filter(|_| *self.show_datums) {
            for drawn in datums::draws(doc, evaluation, scene_extent(self.scene)) {
                for point in drawn.segments {
                    edges
                        .datums
                        .push([point[0] as f32, point[1] as f32, point[2] as f32]);
                }
            }
        }
        if let Some(Ok(drawn)) = self.profile_preview {
            let plane = sketch::form_plane();
            // ONE size for every loop in the preview, from the whole
            // picture's extent: marks that each scaled to their own
            // loop would draw a bore's crosses smaller than its
            // outer's for no reason a reader could name.
            let tick = tip_mark(&drawn.loops);
            for polyline in &drawn.loops {
                let points = &polyline.points;
                // A CLOSED loop's segment list wraps — the last point
                // joins the first, which is the same thing
                // `ProfileLoop` means by being closed by construction.
                // An OPEN one's must not: the leg back to the start is
                // the provisional close `sketch::preview` walked the
                // chain under and nobody authored, so the wrap is
                // dropped and what is drawn is the authored legs
                // exactly. That is the whole of "a path draws while it
                // is still being written".
                let segments = if polyline.closed {
                    points.len()
                } else {
                    points.len().saturating_sub(1)
                };
                let mut segment = |a: [f64; 2], b: [f64; 2]| {
                    for [x, y] in [a, b] {
                        let world = plane.to_world(pncad::geom_core::Point2::new(x, y));
                        edges
                            .preview
                            .push([world.x as f32, world.y as f32, world.z as f32]);
                    }
                };
                for index in 0..segments {
                    segment(points[index], points[(index + 1) % points.len()]);
                }
                // **The directed point at each step.** A tip is a
                // position and, once a verb has bound one, a
                // direction — the pair the lattice calls a directed
                // point, and the thing a person composing a chain is
                // actually reasoning about. The polyline alone shows
                // where the chain went and not where its steps ARE:
                // an arc's flattening puts a dozen indistinguishable
                // points along one leg, which is why
                // `PreviewLoop::vertices` says which of them the loop
                // owns.
                //
                // Each is drawn as a small cross with a tick along the
                // heading. The heading is taken from the polyline
                // itself rather than from bulge arithmetic: the next
                // flattened point IS the tangent to within the chord
                // tolerance, and a second derivation of a direction is
                // a second thing to get wrong.
                for &at in &polyline.vertices {
                    let here = points[at];
                    let Some([dx, dy]) = heading(points, at, polyline.closed) else {
                        continue;
                    };
                    // Both marks are drawn ACROSS the heading, never
                    // along it. A tick that ran along the chain would
                    // lie on the leg already drawn there and be
                    // invisible on every vertex but an open chain's
                    // last — which is the one place a reader needs it
                    // least.
                    let (nx, ny) = (-dy, dx);
                    let at_offset = |along: f64, across: f64| {
                        [
                            here[0] + dx * along * tick + nx * across * tick,
                            here[1] + dy * along * tick + ny * across * tick,
                        ]
                    };
                    // The position: a tick through the point, square
                    // to the path.
                    segment(at_offset(0.0, -0.5), at_offset(0.0, 0.5));
                    // The direction: an arrowhead just ahead of it,
                    // opening backward, so the pair reads as "here,
                    // going that way".
                    let tip = at_offset(1.0, 0.0);
                    segment(tip, at_offset(0.2, 0.45));
                    segment(tip, at_offset(0.2, -0.45));
                }
            }
        }

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

        // **The ground first, then the picture on it.** The pane
        // allocates its rectangle and the paint callback fills only
        // what the model covers, so without this the pixels around a
        // part are whatever the window happened to be cleared to —
        // the toolkit's colour, not the palette's. The palette states
        // it (`Theme::ground`) and this is the one place it is drawn.
        ui.painter()
            .rect_filled(rect, 0.0, chrome(self.theme.ground));
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            ViewportCallback {
                scene: Arc::clone(self.scene),
                revision: self.revision,
                view_projection: to_f32(&matrix),
                light_direction: LIGHT_DIRECTION,
                theme: self.theme,
                viewport_px: [viewport.width_px as f32, viewport.height_px as f32],
                pixels_per_point: pixels_per_point as f32,
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
                format!("{} {GLYPH_ROOT}", row.kind)
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
        let Some(tool) = self.tools.mate().cloned() else {
            if ui.button("Mate tool…").clicked() {
                // ONE modal tool at a time — `Tools::open` closes
                // whatever was open, the rule and its argument living
                // in that value rather than at each activation.
                self.tools.open(ToolKind::Mate);
            }
            return;
        };
        ui.label(ToolKind::Mate.says(&"pick two faces in the viewport"));
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
                            Err(error) => {
                                *self.status = Some(ToolKind::Mate.says(&error));
                            }
                        }
                    }
                    _ => {
                        *self.status = Some(
                            ToolKind::Mate.says(&"no landed evaluation to derive frames from"),
                        );
                    }
                }
            }
            if ui.button("Cancel").clicked() {
                close = true;
            }
        });
        if close {
            self.tools.close();
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
        // The combining tools sit in their own section (GAUTH-4):
        // everything above makes a body out of nothing, everything
        // here takes bodies that exist and makes another one.
        ui.collapsing("Combine bodies", |ui| {
            self.boolean_tool_ui(ui);
            ui.separator();
            self.split_tool_ui(ui);
            ui.separator();
            self.transform_tool_ui(ui);
            ui.separator();
            self.pattern_tool_ui(ui);
        });
        // The blend tools sit in their own section (GAUTH-5): they
        // take a body that exists and reshape its EDGES, which is a
        // third kind of move again — and the only one whose picks are
        // a set rather than a seat.
        ui.collapsing("Blend edges", |ui| {
            self.blend_tool_ui(ui);
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
        ui.horizontal(|ui| {
            unit_vec3_row(
                ui,
                match kind {
                    DatumKind::Point => "position",
                    DatumKind::Plane | DatumKind::Axis => "origin",
                },
                self.drafts.length_unit.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.datum_origin,
            );
            length_picker(ui, "datum_origin", &mut self.drafts.length_unit);
        });
        match kind {
            DatumKind::Plane => vec3_row(
                ui,
                "normal",
                UNIT_DRAG_SPEED,
                &mut self.drafts.datum_direction,
            ),
            DatumKind::Axis => vec3_row(
                ui,
                "direction",
                UNIT_DRAG_SPEED,
                &mut self.drafts.datum_direction,
            ),
            DatumKind::Point => {}
        }
        if ui.button("Add datum").clicked() {
            // The origin is a Length triple in the form's notation; a
            // normal or a direction is dimensionless and has none.
            let datum = (|| -> Result<DatumSpec, DimensionError> {
                let origin = self.drafts.lengths(self.drafts.datum_origin)?;
                Ok(match kind {
                    DatumKind::Plane => DatumSpec::Plane {
                        origin,
                        normal: scalars(self.drafts.datum_direction)?,
                    },
                    DatumKind::Axis => DatumSpec::Axis {
                        origin,
                        direction: scalars(self.drafts.datum_direction)?,
                    },
                    DatumKind::Point => DatumSpec::Point { position: origin },
                })
            })();
            match datum {
                Ok(datum) => self.ops.push(SessionOp::AddDatum { datum }),
                // The add-datum form is not a seated TOOL, so it has
                // no `ToolKind` to compose the prefix — the form's own
                // name is the sentence's subject here.
                Err(error) => *self.status = Some(format!("add datum: {error}")),
            }
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
        *self.profile_form_drawn = true;
        ui.horizontal(|ui| {
            ui.label("profile");
            for (shape, label) in ShapeKind::ALL {
                ui.radio_value(&mut self.drafts.profile_shape, Some(shape), label);
            }
            ui.weak("on the world XY plane");
        });
        let shape = self.drafts.profile_shape;
        let mut blocked: Option<&'static str> = None;
        match shape {
            // No shape chosen: the form is at rest. It says what it is
            // waiting for and draws nothing — no fields to fill in for
            // a shape nobody picked, and no preview in the viewport.
            None => blocked = Some("choose a shape to add"),
            Some(ShapeKind::Circle) => {
                let unit = self.drafts.length_unit.def();
                ui.horizontal(|ui| {
                    ui.label("centre");
                    unit_field(
                        ui,
                        unit,
                        FIELD_DRAG_SPEED,
                        &mut self.drafts.profile_centre[0],
                    );
                    unit_field(
                        ui,
                        unit,
                        FIELD_DRAG_SPEED,
                        &mut self.drafts.profile_centre[1],
                    );
                    ui.label("radius");
                    unit_field(ui, unit, FIELD_DRAG_SPEED, &mut self.drafts.profile_radius);
                    length_picker(ui, "profile_circle", &mut self.drafts.length_unit);
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.drafts.profile_bored, "with bore");
                    if self.drafts.profile_bored {
                        ui.label("bore radius");
                        unit_field(ui, unit, FIELD_DRAG_SPEED, &mut self.drafts.profile_bore);
                    }
                });
                if self.drafts.profile_bored
                    && self.drafts.profile_bore >= self.drafts.profile_radius
                {
                    blocked = Some(
                        "the bore must be smaller than the radius — which loop is the \
                         hole is decided by containment, so a larger bore would swap \
                         the roles rather than refuse",
                    );
                }
            }
            Some(ShapeKind::Rectangle) => {
                let unit = self.drafts.length_unit.def();
                ui.horizontal(|ui| {
                    ui.label("width");
                    unit_field(
                        ui,
                        unit,
                        FIELD_DRAG_SPEED,
                        &mut self.drafts.profile_extent[0],
                    );
                    ui.label("height");
                    unit_field(
                        ui,
                        unit,
                        FIELD_DRAG_SPEED,
                        &mut self.drafts.profile_extent[1],
                    );
                    length_picker(ui, "profile_rectangle", &mut self.drafts.length_unit);
                });
            }
            Some(ShapeKind::Path) => {
                self.path_steps_ui(ui);
                // A chain with no steps is a form waiting for its
                // first one, not a chain that fails to close. Without
                // this the empty list drew the lattice's own refusal
                // about a program nobody had started writing.
                if self.drafts.profile_path.is_empty() {
                    blocked = Some("add a step to the chain");
                }
            }
        }
        if let Some(reason) = blocked {
            ui.weak(reason);
        }
        // **What the loops would draw, said before they are
        // authored.** The preview ran the commit door's own ladder,
        // so a refusal here is the refusal the button would get —
        // which is why the button waits on it rather than letting a
        // reader find out by clicking.
        // **A form at rest reports no preview.** With no shape chosen,
        // or a path chain with no steps, there are no loops to have an
        // opinion about — and a refusal about nothing would be a line
        // of text arguing with the "choose a shape" the form has just
        // said. `blocked` already carries that sentence, and it is
        // what disables the commit; this only keeps a second one from
        // contradicting it.
        let at_rest = shape.is_none() || self.drafts.profile_path.is_empty() && blocked.is_some();
        let refused = match self.profile_preview.as_ref().filter(|_| !at_rest) {
            // The first frame this form is on screen: the latch has
            // not asked for a preview yet, so there is nothing
            // honest to say about one. The commit door is still the
            // judge, so the button is not held for a frame either.
            None => false,
            Some(Ok(drawn)) if drawn.has_open_chain() => {
                // **Drawn, and still not committable.** The chain is
                // in the viewport (`sketch::preview` walks it under a
                // provisional close) so the shape can be looked at
                // while it is written; what it is not yet is a loop,
                // and the commit door refuses a program that does not
                // close. Saying which of the two this is beats a
                // disabled button with a lattice refusal beside it.
                ui.weak("the chain does not close yet — its last step has to target the start");
                true
            }
            Some(Ok(drawn)) => {
                if let Some(invalid) = &drawn.invalid {
                    ui.colored_label(
                        chrome(self.theme.unresolved),
                        format!("does not validate: {invalid}"),
                    );
                    true
                } else {
                    ui.weak(format!(
                        "{} loop(s), drawn in the viewport",
                        drawn.loops.len()
                    ));
                    false
                }
            }
            Some(Err(error)) => {
                // **Unfinished is not wrong.** The end-of-program arm
                // says only that the chain has no closing verb yet,
                // which is the state every chain passes through while
                // it is being written — a one-point chain reaches the
                // form this way, because there is no leg for the
                // provisional close to be walked over. Every OTHER
                // refusal blames a step somebody actually wrote, and
                // keeps the colour that says so.
                if matches!(error, PreviewError::Transition { verb: None, .. }) {
                    ui.weak(error.to_string());
                } else {
                    ui.colored_label(chrome(self.theme.unresolved), error.to_string());
                }
                true
            }
        };
        if ui
            .add_enabled(
                blocked.is_none() && !refused,
                egui::Button::new("Add profile"),
            )
            .clicked()
        {
            // Lowered HERE rather than in the session: the notation is
            // the form's, and a literal that forgot it between the two
            // is exactly the gap this carries across.
            match self.drafts.profile_programs() {
                Ok(loops) => self.ops.push(SessionOp::AddProfile {
                    plane: sketch::form_plane(),
                    loops,
                }),
                Err(error) => *self.status = Some(format!("add profile: {error}")),
            }
        }
    }

    /// **The path form's step list**: one row per verb, plus the
    /// control that appends another.
    ///
    /// Each row is `N [x] [up] [down] <verb> <the verb's fields>` — a
    /// list a person edits in place, because a chain IS a list and its
    /// order is the whole content. Changing a row's verb replaces the
    /// step with a fresh one of that verb rather than carrying
    /// numbers across: two verbs' fields mean different things (a
    /// `line`'s length is not a `turn`'s angle), so a carried number
    /// would be a guess the form cannot check.
    ///
    /// **The row number is the one the refusals use.** A preview
    /// refusal reads "loop 0 step 2", and a list with nothing written
    /// on it left a reader counting rows to find which one that was.
    /// It is therefore zero-based, matching the sentence rather than
    /// matching what a list of things usually looks like.
    ///
    /// **The verb combo offers only what the lattice admits at that
    /// tip**, and shows the rest greyed with the refusal as their
    /// hover text — [`sketch::admits_at`], which answers by putting
    /// the candidate in front of the same `replay` the commit door
    /// runs. That is deliberately NOT a table here: an earlier version
    /// of this form offered every verb everywhere precisely to avoid
    /// keeping a second copy of the lattice in step by hand, and the
    /// probe is how the offer narrows without one existing. The
    /// admissible set is computed only while a combo is OPEN, so a
    /// closed form pays nothing for it.
    fn path_steps_ui(&mut self, ui: &mut egui::Ui) {
        let length_unit = self.drafts.length_unit.def();
        let angle_unit = self.drafts.angle_unit.def();
        ui.horizontal(|ui| {
            ui.weak("written in");
            length_picker(ui, "path_length", &mut self.drafts.length_unit);
            angle_picker(ui, "path_angle", &mut self.drafts.angle_unit);
        });
        // The row edits are COLLECTED and applied after the loop: a
        // list cannot be reordered or shortened while it is being
        // iterated, and one edit per frame is what keeps two buttons
        // clicked in one frame from compounding into a move nobody
        // asked for.
        let mut remove: Option<usize> = None;
        let mut swap: Option<(usize, usize)> = None;
        // A verb chosen in a row's combo, applied after the loop for
        // the same reason the moves are: the probe that decides which
        // verbs a combo may offer reads the WHOLE list, and it cannot
        // borrow it while a row holds a mutable slice of it.
        let mut rebind: Option<(usize, PathVerb)> = None;
        let mut insert: Option<usize> = None;
        let notation = self.drafts.notation();
        let tol = self.session.tol();
        let last = self.drafts.profile_path.len().saturating_sub(1);
        for index in 0..self.drafts.profile_path.len() {
            let salt = format!("path_step_{index}");
            ui.horizontal(|ui| {
                // Zero-based, because "loop 0 step 2" is.
                ui.weak(format!("{index}"));
                if ui
                    .small_button(GLYPH_REMOVE)
                    .on_hover_text("remove this step")
                    .clicked()
                {
                    remove = Some(index);
                }
                if ui
                    .add_enabled(index > 0, egui::Button::new(GLYPH_UP).small())
                    .on_hover_text("move this step earlier")
                    .clicked()
                {
                    swap = Some((index, index - 1));
                }
                if ui
                    .add_enabled(index < last, egui::Button::new(GLYPH_DOWN).small())
                    .on_hover_text("move this step later")
                    .clicked()
                {
                    swap = Some((index, index + 1));
                }
                // **Insert after this row.** A chain is written in the
                // middle as often as at the end — a leg forgotten
                // between two that exist used to mean appending it and
                // walking it up with the arrows — so every row carries
                // the control, and the last row's is the append.
                //
                // In the row's own control cluster rather than at the
                // far end of it, which is where this first went: a
                // row's width is its verb's, so at the end the `+`
                // sits at a different place on every row and, on the
                // widest, past the edge of a pane that does not scroll
                // sideways. A control that moves under the cursor is
                // worse than one that is not where a reader first
                // looks for it.
                if ui
                    .small_button("+")
                    .on_hover_text("insert a step after this one")
                    .clicked()
                {
                    insert = Some(index + 1);
                }
                let verb = PathVerb::of(&self.drafts.profile_path[index]);
                egui::ComboBox::from_id_salt(("path_verb", index))
                    .selected_text(verb.label())
                    .width(120.0)
                    .show_ui(ui, |ui| {
                        // Asked once per OPEN combo, never per frame:
                        // the probe replays the prefix once per
                        // candidate verb, which is cheap but not free,
                        // and a closed combo has nobody to show it to.
                        let mut chain = self.drafts.profile_path.clone();
                        for option in PathVerb::ALL {
                            chain[index] = option.fresh();
                            let refusal = sketch::admits_at(&chain, index, notation, tol).err();
                            // `add_enabled` on the widget itself, not
                            // an `add_enabled_ui` around it: the
                            // reason a choice is greyed out is told
                            // through `on_disabled_hover_text`, and
                            // that is a `Response`'s door — a region's
                            // response shows nothing.
                            let row = ui.add_enabled(
                                refusal.is_none(),
                                egui::Button::selectable(option == verb, option.label()),
                            );
                            match refusal {
                                Some((state, _refused)) => {
                                    // The label the combo shows, not
                                    // the kernel verb's `Debug`: the
                                    // sentence is about the row a
                                    // reader is looking at.
                                    row.on_disabled_hover_text(format!(
                                        "{} is not well-typed here — the tip is {}",
                                        option.label(),
                                        sketch::tip_state_words(state),
                                    ));
                                }
                                None if row.clicked() && option != verb => {
                                    rebind = Some((index, option));
                                }
                                None => {}
                            }
                        }
                    });
                let step = &mut self.drafts.profile_path[index];
                path_step_fields(ui, &salt, length_unit, angle_unit, step);
            });
        }
        if let Some((index, verb)) = rebind {
            self.drafts.profile_path[index] = verb.fresh();
        }
        if let Some(index) = remove {
            self.drafts.profile_path.remove(index);
        }
        if let Some(at) = insert {
            self.drafts.profile_path.insert(at, fresh_step(at));
        }
        if let Some((from, to)) = swap {
            self.drafts.profile_path.swap(from, to);
        }
        ui.horizontal(|ui| {
            // **"Add step" only when there is no row to insert after.**
            // Once the list has rows, every one of them carries a `+`
            // that inserts after it — including the last, which is the
            // append — so a second control at the bottom would be the
            // same move spelled twice.
            //
            // There is no verb picker beside it either. It duplicated
            // the row combo one row down: whatever the new step is,
            // the way to change it is the same control either way, and
            // a second one only asked the question a frame earlier.
            if self.drafts.profile_path.is_empty() {
                if ui.button("Add step").clicked() {
                    self.drafts.profile_path.push(fresh_step(0));
                }
            } else if ui.button("Clear").clicked() {
                self.drafts.profile_path.clear();
            }
        });
    }

    /// The extrude form: the current selection is the profile (a tree
    /// pick, or a face pick whose feature is one — `Selection::node`),
    /// one distance field, one [`SessionOp::AddExtrude`] on commit.
    /// A selection that is not a profile refuses typed at the door
    /// and lands on the status line.
    fn extrude_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("extrude");
            ui.label("distance");
            unit_field(
                ui,
                self.drafts.length_unit.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.extrude_distance,
            );
            length_picker(ui, "extrude_distance", &mut self.drafts.length_unit);
        });
        match self.session.selection().node() {
            Some(node) => {
                if ui.button(format!("Extrude feature {}", node.0)).clicked() {
                    match self.drafts.length(self.drafts.extrude_distance) {
                        Ok(distance) => self.ops.push(SessionOp::AddExtrude {
                            profile: node,
                            distance,
                        }),
                        Err(error) => *self.status = Some(format!("extrude: {error}")),
                    }
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
    /// edit — the same chrome shape as every other seated tool.
    fn revolve_tool_ui(&mut self, ui: &mut egui::Ui) {
        // A copy of the small tool value, for the reason the mate
        // panel takes one: the panel reads it while pushing ops and
        // closing the tool; the authoritative copy is only ever
        // replaced whole.
        let Some(tool) = self.tools.revolve() else {
            if ui.button("Revolve tool…").clicked() {
                // ONE modal tool at a time — `Tools::open` closes
                // whatever was open, the rule and its argument living
                // in that value rather than at each activation.
                self.tools.open(ToolKind::Revolve);
            }
            return;
        };
        ui.label(ToolKind::Revolve.says(&"pick the profile, then the axis"));
        ui.weak(seat_line(&[
            (Seat::RevolveProfile, tool.profile()),
            (Seat::RevolveAxis, tool.axis()),
        ]));
        ui.horizontal(|ui| {
            ui.label("angle");
            unit_field(
                ui,
                self.drafts.angle_unit.def(),
                ANGLE_DRAG_SPEED,
                &mut self.drafts.revolve_angle,
            );
            angle_picker(ui, "revolve_angle", &mut self.drafts.angle_unit);
        });
        self.tool_commit_row(ui, "Commit revolve", ToolKind::Revolve, |drafts| {
            Ok(tool.op(drafts.angle(drafts.revolve_angle)?)?)
        });
    }

    /// The boolean tool's panel: activation, the two held picks named
    /// by ROLE, the operation choice, and the one committed edit.
    ///
    /// The role naming is the point of the panel: `subtract` removes
    /// the second pick from the first, so a user who cannot see which
    /// is which cannot author the operation they mean.
    fn boolean_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.boolean() else {
            if ui.button("Boolean tool…").clicked() {
                self.tools.open(ToolKind::Boolean);
            }
            return;
        };
        ui.label(ToolKind::Boolean.says(&"pick the first body, then the second"));
        ui.weak(seat_line(&[
            (Seat::OperandA, tool.a()),
            (Seat::OperandB, tool.b()),
        ]));
        ui.horizontal(|ui| {
            ui.label("operation");
            for (op, label) in BOOLEAN_OPS {
                ui.radio_value(&mut self.drafts.boolean_op, op, label);
            }
        });
        if self.drafts.boolean_op == BooleanOp::Subtract {
            ui.weak("subtract removes the second pick from the first");
        }
        self.tool_commit_row(ui, "Commit boolean", ToolKind::Boolean, |drafts| {
            Ok(tool.op(drafts.boolean_op)?)
        });
    }

    /// The split tool's panel: a body pick, a datum-plane pick, and
    /// the one committed edit.
    fn split_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.split() else {
            if ui.button("Split tool…").clicked() {
                self.tools.open(ToolKind::Split);
            }
            return;
        };
        ui.label(ToolKind::Split.says(&"pick the body, then the datum plane"));
        ui.weak(seat_line(&[
            (Seat::SplitTarget, tool.target()),
            (Seat::SplitPlane, tool.plane()),
        ]));
        self.tool_commit_row(ui, "Commit split", ToolKind::Split, |_| Ok(tool.op()?));
    }

    /// The transform tool's panel: one body pick plus the placement
    /// fields, and the one committed edit.
    fn transform_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.transform() else {
            if ui.button("Transform tool…").clicked() {
                self.tools.open(ToolKind::Transform);
            }
            return;
        };
        ui.label(ToolKind::Transform.says(&"pick the body to place"));
        ui.weak(seat_line(&[(Seat::TransformBody, tool.input())]));
        ui.horizontal(|ui| {
            unit_vec3_row(
                ui,
                "translation",
                self.drafts.length_unit.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.transform_translation,
            );
            length_picker(ui, "transform_translation", &mut self.drafts.length_unit);
        });
        vec3_row(
            ui,
            "rotation axis",
            UNIT_DRAG_SPEED,
            &mut self.drafts.transform_axis,
        );
        ui.horizontal(|ui| {
            ui.label("rotation angle");
            unit_field(
                ui,
                self.drafts.angle_unit.def(),
                ANGLE_DRAG_SPEED,
                &mut self.drafts.transform_angle,
            );
            angle_picker(ui, "transform_angle", &mut self.drafts.angle_unit);
        });
        self.tool_commit_row(ui, "Commit transform", ToolKind::Transform, |drafts| {
            Ok(tool.op(
                drafts.lengths(drafts.transform_translation)?,
                scalars(drafts.transform_axis)?,
                drafts.angle(drafts.transform_angle)?,
            )?)
        });
    }

    /// The pattern tool's panel: a body pick, a rule choice with its
    /// fields, the axis pick the circular rule needs, and the one
    /// committed edit.
    fn pattern_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.pattern() else {
            if ui.button("Pattern tool…").clicked() {
                self.tools.open(ToolKind::Pattern);
            }
            return;
        };
        ui.label(ToolKind::Pattern.says(&"pick the body, then (circular) the axis"));
        ui.weak(seat_line(&[
            (Seat::PatternBody, tool.input()),
            (Seat::PatternAxis, tool.axis()),
        ]));
        ui.horizontal(|ui| {
            ui.label("rule");
            for (kind, label) in PatternKindChoice::ALL {
                ui.radio_value(&mut self.drafts.pattern_kind, kind, label);
            }
        });
        ui.horizontal(|ui| {
            ui.label("count");
            // Clamped at one instance: a count is a structural slot
            // and a non-positive one refuses at evaluation, so the
            // form does not offer to author a node that cannot build.
            ui.add(
                egui::DragValue::new(&mut self.drafts.pattern_count)
                    .speed(COUNT_DRAG_SPEED)
                    .range(MIN_PATTERN_COUNT..=i64::MAX),
            );
        });
        match self.drafts.pattern_kind {
            PatternKindChoice::Linear => {
                vec3_row(
                    ui,
                    "direction",
                    UNIT_DRAG_SPEED,
                    &mut self.drafts.pattern_direction,
                );
                ui.horizontal(|ui| {
                    ui.label("spacing");
                    unit_field(
                        ui,
                        self.drafts.length_unit.def(),
                        FIELD_DRAG_SPEED,
                        &mut self.drafts.pattern_spacing,
                    );
                    length_picker(ui, "pattern_spacing", &mut self.drafts.length_unit);
                });
            }
            PatternKindChoice::Circular => {
                ui.horizontal(|ui| {
                    ui.label("step");
                    unit_field(
                        ui,
                        self.drafts.angle_unit.def(),
                        ANGLE_DRAG_SPEED,
                        &mut self.drafts.pattern_step,
                    );
                    angle_picker(ui, "pattern_step", &mut self.drafts.angle_unit);
                });
            }
        }
        self.tool_commit_row(
            ui,
            "Commit pattern",
            ToolKind::Pattern,
            |drafts| match drafts.pattern_kind {
                PatternKindChoice::Linear => Ok(tool.linear_op(
                    drafts.pattern_count,
                    scalars(drafts.pattern_direction)?,
                    drafts.length(drafts.pattern_spacing)?,
                )?),
                PatternKindChoice::Circular => {
                    Ok(tool.circular_op(drafts.pattern_count, drafts.angle(drafts.pattern_step)?)?)
                }
            },
        );
    }

    /// The blend tool's panel: activation, the freeze sentence, the
    /// live count of held edges, the all-edges affordance, the kind
    /// choice with its one Length field, and the one committed edit.
    ///
    /// **The freeze sentence is not decoration.** #217 makes the
    /// selection a commitment — a later edit that adds an edge does
    /// not extend the blend, and one that strands a picked edge
    /// refuses on the node — and the moment a user needs to know that
    /// is while they are choosing the set, so it is stated here rather
    /// than only in the node's docs.
    fn blend_tool_ui(&mut self, ui: &mut egui::Ui) {
        // **Read, never cloned.** The tool holds a SET, so copying it
        // per frame is per-frame work proportional to the picks; what
        // the panel actually needs is two small values, and the commit
        // door is re-borrowed at the click.
        let Some((target, count)) = self.tools.blend().map(|tool| (tool.target(), tool.count()))
        else {
            if ui.button("Blend tool…").clicked() {
                self.tools.open(ToolKind::Blend);
            }
            return;
        };
        ui.label(ToolKind::Blend.says(&"pick the edges to blend"));
        ui.weak(FREEZE_NOTE);
        ui.weak(match target {
            Some(target) => format!("{count} edges picked on {target}"),
            None => "no edges picked yet".to_owned(),
        });
        self.all_edges_row(ui, target);
        ui.horizontal(|ui| {
            ui.label("blend");
            for (kind, label) in BlendKindChoice::ALL {
                ui.radio_value(&mut self.drafts.blend_kind, kind, label);
            }
        });
        ui.horizontal(|ui| {
            ui.label(self.drafts.blend_kind.size_label());
            unit_field(
                ui,
                self.drafts.length_unit.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.blend_size,
            );
            length_picker(ui, "blend_size", &mut self.drafts.length_unit);
        });
        self.blend_commit_row(ui, count);
    }

    /// **The all-edges affordance** — `editor_core::all_edges` through
    /// the tool's own loading door, which stores what it returns as an
    /// ordinary frozen set: indistinguishable from clicking each edge,
    /// which is exactly why `Node::Fillet` has no every-edge variant.
    ///
    /// The body it is about is the one the held edges are on; with
    /// nothing held, the DRAWN BODY the current selection is a pick
    /// on. So "click the body, press the button" works from a standing
    /// start, and once picking has begun the button cannot move the
    /// tool to another body behind the user's back.
    ///
    /// **A tree click does not name a body.** `Selection::Node` is a
    /// feature, and a feature is not a `(node, body)` pair — the body
    /// index a load must narrow by only exists on a pick made against
    /// something DRAWN. Assuming body 0 there was a guess that read
    /// wrong on exactly the nodes the narrowing matters for, so the
    /// button is disabled instead and says what it wants.
    fn all_edges_row(&mut self, ui: &mut egui::Ui, held: Option<BlendTarget>) {
        let target = held.or_else(|| BlendTarget::of_selection(self.session.selection()));
        let ready = target.zip(self.session.evaluation()).zip(self.index);
        let Some(((target, eval), index)) = ready else {
            ui.add_enabled(false, egui::Button::new("Select all edges"))
                .on_disabled_hover_text(
                    "click an edge or a face of the body first, and let it evaluate —                      a feature picked in the tree does not say which body",
                );
            return;
        };
        let clicked = ui
            .button("Select all edges")
            .on_hover_text(
                "every edge of this body as it stands now, stored as a frozen set —                  whether the kernel can BLEND that set is its own answer, on the node's badge",
            )
            .clicked();
        if !clicked {
            return;
        }
        // The load reads the LANDED evaluation and the index, and
        // writes tool state: no document edit, so no op —
        // `BlendTool::load_all_edges` is the typed operation, callable
        // with no renderer, and this is its one widget.
        let event = self
            .tools
            .blend_mut()
            .and_then(|tool| tool.load_all_edges(target, eval, index));
        if let Some(event) = event {
            *self.status = Some(ToolKind::Blend.says(&event));
        }
    }

    /// The blend tool's commit/cancel row — [`ViewerBehavior::tool_commit_row`]'s
    /// rules, spelled here because this tool's commit door refuses in
    /// its own vocabulary ([`BlendError`]) rather than in the seats'.
    ///
    /// Both halves of the close rule are unchanged: the op is QUEUED
    /// and the application closes the tool when the edit actually
    /// commits, so a refusal at the session door leaves every picked
    /// edge in place to correct; Cancel closes at once.
    ///
    /// **`Clear picks` is this tool's third button and the seated
    /// tools' second**: a set-valued tool needs a way to start the
    /// PICKS over without closing the form beside them, which is what
    /// `BlendTool::clear` is and what Cancel is not — Cancel replaces
    /// the whole tool value.
    fn blend_commit_row(&mut self, ui: &mut egui::Ui, count: usize) {
        let mut close = false;
        ui.horizontal(|ui| {
            if ui.button("Commit blend").clicked() {
                match self.drafts.length(self.drafts.blend_size) {
                    Ok(size) => {
                        let op: Option<Result<SessionOp, BlendError>> =
                            self.tools.blend().map(|tool| match self.drafts.blend_kind {
                                BlendKindChoice::Fillet => tool.fillet_op(size.clone()),
                                BlendKindChoice::Chamfer => tool.chamfer_op(size),
                            });
                        match op {
                            Some(Ok(op)) => self.ops.push(op),
                            Some(Err(error)) => {
                                *self.status = Some(ToolKind::Blend.says(&error));
                            }
                            None => {}
                        }
                    }
                    Err(error) => *self.status = Some(ToolKind::Blend.says(&error)),
                }
            }
            if ui
                .add_enabled(count > 0, egui::Button::new("Clear picks"))
                .on_hover_text("drop every picked edge and start on any body")
                .clicked()
                && let Some(tool) = self.tools.blend_mut()
            {
                tool.clear();
            }
            if ui.button("Cancel").clicked() {
                close = true;
            }
        });
        if close {
            self.tools.close();
        }
    }

    /// **The commit/cancel row every combining tool ends with**, and
    /// the one place their two halves of the close rule live.
    ///
    /// The op is QUEUED and the tool is not closed here: the
    /// application closes it when the op actually commits
    /// (`perform_batch`), so a refusal at the session door leaves the
    /// held picks in place to correct instead of costing all of them.
    /// Cancel closes immediately, being the door that means "drop
    /// these picks".
    fn tool_commit_row(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        kind: ToolKind,
        op: impl FnOnce(&Drafts) -> Result<SessionOp, CommitFault>,
    ) {
        let mut close = false;
        ui.horizontal(|ui| {
            if ui.button(label).clicked() {
                match op(self.drafts) {
                    Ok(op) => self.ops.push(op),
                    Err(error) => *self.status = Some(kind.says(&error)),
                }
            }
            if ui.button("Cancel").clicked() {
                close = true;
            }
        });
        if close {
            self.tools.close();
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
        let unit = props::rendering_unit(row.dimension, row.unit);
        // `Count` is the one row with no unit at all (an instance count
        // is a number, not a quantity), and its factor would be 1.0
        // anyway — so the absence is an identity here, not a fallback.
        let written = |v: f64| unit.map_or(v, |u| props::in_written(v, u));
        let canonical = |v: f64| unit.map_or(v, |u| props::from_written(v, u));
        // A slot that did not evaluate still has SOURCE to edit — it
        // is the slot most likely to need it — so the field is drawn
        // for it too, over the one number it does not have. The fault
        // itself is said beside the field.
        if let Err(ref error) = row.value {
            ui.weak(format!("{error}"));
        }
        let mut number = written(match row.value {
            Ok(value) => value.as_f64(),
            Err(_) => 0.0,
        });
        // The drag speed is in WRITTEN units, so it travels through
        // the same conversion the value does ([`unit_field`] carries
        // the rule) — and it is `FIELD_DRAG_SPEED`, the same tick the
        // creation forms use, rather than a second number for the
        // same gesture. A structural slot steps in whole units: what
        // it holds is a count.
        let speed = if row.structural {
            1.0
        } else {
            written(drag_tick(row.dimension))
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
            canonical(number),
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
                let value = SlotValue::of(row.dimension, canonical(written));
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
            .map(|row| props::rendering_unit(row.dimension, row.unit))
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
                                unit: option,
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
                result.wording(props::rendering_unit(row.dimension, row.unit))
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
                // A document parameter's authored unit is stored but
                // not yet read here (`props`' module docs name the
                // asymmetry), so its range reads in the canonical one.
                ui.weak(result.wording(props::rendering_unit(dimension, None)));
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
        // **Datum visibility, and why it is a switch at all.**
        // Construction geometry is drawn over the part, which is
        // where it has to be for a plane to say what it cuts — and it
        // is also in the way once a document has several. A view
        // setting rather than a document one: which datums exist is
        // the recipe's business, and whether this window draws them is
        // this window's.
        ui.checkbox(self.show_datums, "show datums");
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

/// One labeled row of three draggable components — every creation
/// form's vector fields (a datum's origin and normal, a placement's
/// translation and rotation axis, a pattern's direction).
///
/// **The speed is the caller's** because the unit is: a metre field
/// and a dimensionless direction component want drag rates two orders
/// of magnitude apart, and one shared rate makes one of them
/// undraggable. [`FIELD_DRAG_SPEED`] and [`UNIT_DRAG_SPEED`] are the
/// two values in use.
fn vec3_row(ui: &mut egui::Ui, label: &str, speed: f64, value: &mut [f64; 3]) {
    ui.horizontal(|ui| {
        ui.label(label);
        for component in value {
            ui.add(egui::DragValue::new(component).speed(speed));
        }
    });
}

/// **One dimensioned field of a creation form.**
///
/// The draft behind it is CANONICAL (metres, radians) and the field
/// is what that value looks like written in `unit` — the property
/// panel's own rule (`props::in_written` / `props::from_written`, the
/// text door's one multiply), applied to the forms so a number typed
/// into a form and the same number typed into a panel field mean the
/// same thing.
///
/// Held canonical rather than as-typed for the reason the panel holds
/// it that way: switching the unit is a change of NOTATION, and a
/// draft that stored what was typed would silently become a different
/// length when the picker moved.
///
/// **The drag speed travels through the same conversion**, which is
/// the half of this that is easy to leave out: a tick in metres
/// applied to a field showing millimetres is the same gesture made a
/// thousand times finer by a change of notation.
fn unit_field(ui: &mut egui::Ui, unit: UnitDef, speed: f64, canonical: &mut f64) {
    named_field(ui, "", unit, speed, canonical);
}

/// [`unit_field`] with the quantity's NAME written into the field
/// itself.
///
/// A prefix rather than a `Label` beside it, and that is the point: a
/// path step's row is a horizontal strip of controls, and the labels
/// that used to sit between them belonged to whichever field a reader
/// guessed. `arc_fillet` is the case that made it matter — an arc
/// radius and a fillet radius, both written `r`, both in the same row,
/// with nothing saying which was which. A prefix cannot drift away
/// from its field.
///
/// The name is the QUANTITY, never the unit: the picker beside the
/// form says the unit, and a second statement of it here would be
/// free to disagree ([`length_picker`]'s own rule).
fn named_field(ui: &mut egui::Ui, name: &str, unit: UnitDef, speed: f64, canonical: &mut f64) {
    let mut written = props::in_written(*canonical, unit);
    let mut field = egui::DragValue::new(&mut written).speed(props::in_written(speed, unit));
    if !name.is_empty() {
        field = field.prefix(format!("{name} "));
    }
    let response = ui.add(field);
    // Written back only on a real edit: an untouched field would
    // otherwise round-trip its value through a divide and a multiply
    // every frame, which is a drift nobody asked for.
    if response.changed() {
        *canonical = props::from_written(written, unit);
    }
}

/// A dimensionless field with its own name written in — the scalar
/// twin of [`named_field`], for the components and bulges that carry
/// no unit at all.
fn named_scalar(ui: &mut egui::Ui, name: &str, speed: f64, value: &mut f64) {
    ui.add(
        egui::DragValue::new(value)
            .speed(speed)
            .prefix(format!("{name} ")),
    );
}

/// The vector twin of [`unit_field`] — one label, three components,
/// one unit.
fn unit_vec3_row(ui: &mut egui::Ui, label: &str, unit: UnitDef, speed: f64, value: &mut [f64; 3]) {
    ui.horizontal(|ui| {
        ui.label(label);
        for component in value {
            unit_field(ui, unit, speed, component);
        }
    });
}

/// Two Length fields, one point of the sketch frame — each carrying
/// the axis it is, because a row of a path form holds several points
/// and a bare pair of numbers says which of them it belongs to only
/// by position.
fn point_fields(ui: &mut egui::Ui, unit: UnitDef, point: &mut [f64; 2]) {
    for (axis, component) in ["x", "y"].into_iter().zip(point) {
        named_field(ui, axis, unit, FIELD_DRAG_SPEED, component);
    }
}

/// A path verb's target: the entry vertex (which CLOSES the loop), or
/// an authored point.
///
/// The two are one control because they are one decision — where this
/// leg ends — and `Start` is not a point somebody could type: it is
/// the bound entry, and aiming at it is what closing IS in this
/// algebra (`pncad::profile::path`, which has no `close()` alias).
fn target_fields(ui: &mut egui::Ui, unit: UnitDef, target: &mut PathTarget) {
    let closing = matches!(target, PathTarget::Start);
    let mut to_start = closing;
    ui.checkbox(&mut to_start, "to start");
    if to_start != closing {
        *target = if to_start {
            PathTarget::Start
        } else {
            PathTarget::Point([0.01, 0.0])
        };
    }
    if let PathTarget::Point(point) = target {
        point_fields(ui, unit, point);
    }
}

/// Which side of travel an arc's centre sits on.
fn side_picker(ui: &mut egui::Ui, salt: &str, side: &mut ArcSide) {
    egui::ComboBox::from_id_salt(("arc_side", salt))
        .selected_text(match side {
            ArcSide::Left => "left",
            ArcSide::Right => "right",
        })
        .width(64.0)
        .show_ui(ui, |ui| {
            ui.selectable_value(side, ArcSide::Left, "left");
            ui.selectable_value(side, ArcSide::Right, "right");
        });
}

/// Which way round an arc about a named centre travels.
fn winding_picker(ui: &mut egui::Ui, salt: &str, winding: &mut ArcSweep) {
    egui::ComboBox::from_id_salt(("arc_winding", salt))
        .selected_text(match winding {
            ArcSweep::Ccw => "ccw",
            ArcSweep::Cw => "cw",
        })
        .width(64.0)
        .show_ui(ui, |ui| {
            ui.selectable_value(winding, ArcSweep::Ccw, "ccw");
            ui.selectable_value(winding, ArcSweep::Cw, "cw");
        });
}

/// **One arc leg's spec**: which of the six modes, then that mode's
/// own fields.
///
/// Switching the mode REPLACES the spec with a fresh one of the new
/// mode rather than carrying numbers across. The modes do not share a
/// meaning for their fields — a `radius` mode's `r` is a carrier and
/// a `sweep` mode's is the same carrier with a swept angle beside it,
/// but a `via` point is not a radius at all — so a carried number
/// would sometimes be the right one and sometimes be a coincidence,
/// and a form cannot tell which.
fn arc_fields(
    ui: &mut egui::Ui,
    salt: &str,
    role: &str,
    length_unit: UnitDef,
    angle_unit: UnitDef,
    spec: &mut ArcSpec,
) {
    // What to call this arc's own radius. A step can hold TWO arcs and
    // a fillet between them (`arc_fillet_arc`), and every one of the
    // three has a radius: unqualified, all three fields read `r` and
    // the row is unreadable. The caller names the role because only
    // the caller knows which arc of the step this is.
    let radius = if role.is_empty() {
        "r".to_owned()
    } else {
        format!("{role} r")
    };
    let mut mode = ArcMode::of(spec);
    let before = mode;
    egui::ComboBox::from_id_salt(("arc_mode", salt))
        .selected_text(mode.label())
        .width(88.0)
        .show_ui(ui, |ui| {
            for option in ArcMode::ALL {
                ui.selectable_value(&mut mode, option, option.label());
            }
        });
    if mode != before {
        *spec = mode.fresh();
    }
    match spec {
        ArcSpec::Radius { r, side } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            side_picker(ui, salt, side);
        }
        ArcSpec::Bulge { target, b } => {
            target_fields(ui, length_unit, target);
            named_scalar(ui, "bulge", UNIT_DRAG_SPEED, b);
        }
        ArcSpec::Via { q, target } => {
            ui.label("via");
            point_fields(ui, length_unit, q);
            target_fields(ui, length_unit, target);
        }
        ArcSpec::Center { c, winding, target } => {
            ui.label("centre");
            point_fields(ui, length_unit, c);
            winding_picker(ui, salt, winding);
            target_fields(ui, length_unit, target);
        }
        ArcSpec::Sweep { r, side, angle } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            side_picker(ui, salt, side);
            named_field(ui, "sweep", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        ArcSpec::ArcLen { r, side, len } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            side_picker(ui, salt, side);
            named_field(ui, "arc length", length_unit, FIELD_DRAG_SPEED, len);
        }
    }
}

/// **The step a fresh row starts as**, by where it is going.
///
/// `at` at position 0 — nothing else is well-typed at the entry, so
/// offering anything there would be offering a refusal — and `line_to`
/// anywhere after it, which is the verb a chain is mostly made of. It
/// is a starting point and not a judgement: the row's own combo,
/// narrowed to what the lattice admits at that tip, is where it
/// becomes something else.
fn fresh_step(at: usize) -> PathStep {
    if at == 0 {
        PathVerb::At.fresh()
    } else {
        PathVerb::LineTo.fresh()
    }
}

/// **One authoring verb's own fields.**
///
/// Exhaustive on [`PathStep`], like the lowering it feeds: a verb the
/// vocabulary gains has to be given a row here before it compiles,
/// which is the same protection `crate::sketch`'s lowering has and
/// for the same reason — a verb reachable in one and not the other is
/// a verb nobody can use.
fn path_step_fields(
    ui: &mut egui::Ui,
    salt: &str,
    length_unit: UnitDef,
    angle_unit: UnitDef,
    step: &mut PathStep,
) {
    // **Every field says which quantity it is.** The arms below are
    // split further than the lowering's are — `line` and `fillet` both
    // carry one Length and shared an arm — because what a number MEANS
    // is the thing a row has to say, and a shared arm can only give
    // two different quantities one name.
    match step {
        PathStep::At(point) => point_fields(ui, length_unit, point),
        PathStep::ArcContinue(point) => {
            ui.label("through");
            point_fields(ui, length_unit, point);
        }
        PathStep::FarEndTo(point) => {
            ui.label("far end");
            point_fields(ui, length_unit, point);
        }
        PathStep::Angle(angle) => {
            named_field(ui, "angle", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        PathStep::Turn(angle) => {
            named_field(ui, "turn", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        PathStep::Toward { dx, dy } => {
            named_scalar(ui, "dx", UNIT_DRAG_SPEED, dx);
            named_scalar(ui, "dy", UNIT_DRAG_SPEED, dy);
        }
        PathStep::Line(length) => {
            named_field(ui, "length", length_unit, FIELD_DRAG_SPEED, length);
        }
        PathStep::Fillet(radius) => {
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
        }
        PathStep::LineTo(target) | PathStep::TangentArcTo(target) => {
            target_fields(ui, length_unit, target);
        }
        PathStep::ArcTo(spec) => arc_fields(ui, salt, "", length_unit, angle_unit, spec),
        // The two mixed verbs read in the order their names do, so the
        // row is the step spelled left to right.
        PathStep::FilletArc { radius, spec } => {
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
            arc_fields(ui, salt, "arc", length_unit, angle_unit, spec);
        }
        PathStep::ArcFillet { spec, radius } => {
            arc_fields(ui, salt, "arc", length_unit, angle_unit, spec);
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
        }
        PathStep::ArcFilletArc {
            spec,
            radius,
            spec2,
        } => {
            arc_fields(
                ui,
                &format!("{salt}_in"),
                "in arc",
                length_unit,
                angle_unit,
                spec,
            );
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
            arc_fields(
                ui,
                &format!("{salt}_out"),
                "out arc",
                length_unit,
                angle_unit,
                spec2,
            );
        }
        // Structural verbs: the verb IS the whole step.
        PathStep::Tangent | PathStep::Cusp | PathStep::CloseTo => {}
    }
}

/// **The creation forms' written-unit picker.**
///
/// The panel's picker as a form control: the same options
/// (`props::unit_options`, read off the closed unit table) and the
/// same rule about what the label beside a field may say. There is no
/// "nothing chosen" state to fall back from — a form is always
/// authoring in some notation, and says which. **The unit is the picker's
/// to say, not the field's**, which is why the form labels next to
/// these are bare ("radius", not "radius (m)"): a label with the unit
/// baked in is a second place for it to be stated, free to say metres
/// beside a field written in millimetres.
///
/// It differs from the panel's in one way, and deliberately: the
/// panel's picker is an EDIT (`SessionOp::SetSlotUnit` — how a
/// literal that exists is written), while this one only decides how
/// the field beside it reads. Nothing here reaches a document until
/// the form's own commit button.
///
/// **It is drawn after the fields it governs**, so a unit picked now
/// re-writes them on the NEXT frame: the fields were built before
/// this widget ran, and the pick is an input event, so that next
/// frame is the one egui draws in response to it. Drawing it first
/// would close the gap and put the unit above the number it is the
/// unit of, which is the worse trade for a lag nobody can see.
fn length_picker(ui: &mut egui::Ui, salt: &str, chosen: &mut LengthUnit) {
    if let Some(row) = pick_unit(ui, salt, Dimension::Length, chosen.def())
        && let Some(unit) = row.as_length()
    {
        *chosen = unit;
    }
}

/// [`length_picker`]'s angle twin. Two functions rather than one over
/// a dimension, because a length picker that could write a `deg` into
/// its draft is the mismatch the typed views exist to make
/// unrepresentable — the pairing is checked by the compiler here, not
/// by a branch.
fn angle_picker(ui: &mut egui::Ui, salt: &str, chosen: &mut AngleUnit) {
    if let Some(row) = pick_unit(ui, salt, Dimension::Angle, chosen.def())
        && let Some(unit) = row.as_angle()
    {
        *chosen = unit;
    }
}

/// The combo itself: the rows `dimension` admits, with `shown`
/// selected; `Some(row)` when this frame's click chose one.
fn pick_unit(
    ui: &mut egui::Ui,
    salt: &str,
    dimension: Dimension,
    shown: UnitDef,
) -> Option<UnitDef> {
    let options = props::unit_options(dimension);
    if options.is_empty() {
        return None;
    }
    let mut picked = None;
    egui::ComboBox::from_id_salt(("creation_unit", salt))
        .selected_text(shown.symbol())
        // Wide enough for the longest symbol the table carries
        // (`pi rad`) plus the combo's arrow — the panel's width, for
        // the panel's reason.
        .width(72.0)
        .show_ui(ui, |ui| {
            for option in options {
                if ui
                    .selectable_label(shown == option, option.symbol())
                    .clicked()
                {
                    picked = Some(option);
                }
            }
        });
    picked
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
