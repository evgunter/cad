//! **The whole app, driven by label rather than by pixel**: a
//! [`ViewerApp`] in an egui context with no window and no graphics
//! device, fed pointer and key events and read back as the text it
//! painted and where.
//!
//! Module kind: **driver** — part of the app driver: it owns a
//! [`ViewerApp`] and the context its frames run in, and drives them.
//!
//! What a person does at the window is said here the way they would
//! say it: click `Extrude`, type `0.02`, press Enter. A label is found
//! in the frame that was last painted, so a control that is not on
//! screen cannot be clicked, and a click lands where the pointer would
//! have to be. Everything but pixel painting is the real app: the
//! viewport pane allocates its rect and folds the pointer into camera
//! operations and ray picks, and only the paint callback it queues is
//! never run.
//!
//! **Two doors are not the window's.** A file dialog blocks the frame
//! loop on a desktop chooser, so a driven app has none — its Open… and
//! Save As… buttons are disabled with the reason they give on a box
//! with no chooser — and [`Driver::open`] and [`Driver::save`] perform
//! the operations those buttons end in. And the preferences store is
//! the one [`Driver::start`] is handed, so a driven session neither
//! reads nor writes the preferences of the person running it.
//!
//! Behind the crate's `test-support` feature: `tests/` and the
//! `drive` example reach it as `viewer::app::drive`, and the app's
//! own unit tests share its frame and wait helpers.

// Panicking is a test harness's failure mechanism (workspace lint note).
#![allow(clippy::panic)]

use std::path::{Path, PathBuf};

use eframe::egui;
use pncad::geom_core::Tol;

use super::{StartupError, ViewerApp};
use crate::platform::ChooserBackend;
use crate::prefs::file::FileStore;
use crate::session::{DocSession, SessionOp};

/// The window a driven app is laid out in unless told otherwise — the
/// size `run` opens the real window at.
pub const WINDOW: egui::Vec2 = egui::vec2(1280.0, 800.0);

/// The clock's advance per frame, in seconds. Long enough that a
/// collapsing header's animation ends in a few frames, short enough
/// that two clicks one frame apart are still a double click.
const FRAME_SECONDS: f64 = 0.05;

/// How many frames [`settle`] paints before it calls the app stuck.
const SETTLE_FRAMES: usize = 3000;

/// **What one frame PAINTED and WHERE** — one entry per `Shape::Text`
/// the frame emitted, in paint order.
///
/// What a frame said and where it put it come off the same walk
/// because they come off the same shape, and a layout defect shows up
/// only in the second: a sentence drawn past the right edge of its
/// pane and a sentence wrapped inside it paint the same string.
#[derive(Clone, Debug)]
pub struct Landed {
    /// The galley's whole text.
    pub text: String,
    /// **Where the LAYOUT put the galley**: `pos` plus
    /// `egui::Galley::size`, so the leading space egui indents a first
    /// row by is INSIDE it. That is the box a pointer has to be in for
    /// the widget to be hit.
    pub allocated: egui::Rect,
    /// **Where the READER's eye finds the glyphs**: one rect per row of
    /// the galley, each EXCLUDING that leading space, so `left()` is
    /// the row's first glyph rather than the indent in front of it.
    pub rows: Vec<egui::Rect>,
    /// **The colour its glyphs were painted in**, when one colour
    /// painted all of them; `None` for a galley painted in more than
    /// one. Read as the painter reads it: the shape's override first,
    /// then each section's own colour, with egui's placeholder standing
    /// for the shape's fallback.
    pub ink: Option<egui::Color32>,
    /// **The clip rect it was painted under**: a row's part outside it
    /// is not on screen.
    pub clip: egui::Rect,
}

impl Landed {
    /// The part of [`Self::allocated`] that is on screen, if any.
    #[must_use]
    pub fn visible(&self) -> Option<egui::Rect> {
        let shown = self.allocated.intersect(self.clip);
        (shown.width() > 0.0 && shown.height() > 0.0).then_some(shown)
    }
}

/// [`Landed`] for every `Shape::Text` in a tree of shapes.
#[must_use]
pub fn landed_in(shapes: &[egui::epaint::ClippedShape]) -> Vec<Landed> {
    fn ink(text: &egui::epaint::TextShape) -> Option<egui::Color32> {
        let mut colours = text.galley.job.sections.iter().map(|section| {
            text.override_text_color.unwrap_or(
                if section.format.color == egui::Color32::PLACEHOLDER {
                    text.fallback_color
                } else {
                    section.format.color
                },
            )
        });
        let first = colours.next()?;
        colours.all(|colour| colour == first).then_some(first)
    }
    fn walk(shape: &egui::Shape, clip: egui::Rect, out: &mut Vec<Landed>) {
        match shape {
            egui::Shape::Text(text) => out.push(Landed {
                text: text.galley.text().to_owned(),
                allocated: egui::Rect::from_min_size(text.pos, text.galley.size()),
                rows: text
                    .galley
                    .rows
                    .iter()
                    .map(|row| {
                        row.rect_without_leading_space()
                            .translate(text.pos.to_vec2())
                    })
                    .collect(),
                ink: ink(text),
                clip,
            }),
            egui::Shape::Vec(inner) => {
                for shape in inner {
                    walk(shape, clip, out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, clipped.clip_rect, &mut out);
    }
    out
}

/// What one frame of the real app showed: its text, and the rect of
/// the viewport's paint callback when the viewport was drawn.
struct Painted {
    landed: Vec<Landed>,
    viewport: Option<egui::Rect>,
}

/// One frame of the real app, in a `window`-sized window, with `time`
/// and `events` as its input.
fn painted(
    ctx: &egui::Context,
    app: &mut ViewerApp,
    frame: &mut eframe::Frame,
    window: egui::Vec2,
    time: Option<f64>,
    events: Vec<egui::Event>,
) -> Painted {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, window)),
        time,
        events,
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        eframe::App::ui(app, ui, frame);
    });
    // Nothing here paints, so the frame's texture delta is dropped
    // rather than uploaded; `TexturesDelta` panics on drop until one
    // is taken.
    output.textures_delta.clear();
    let viewport = output
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Callback(callback) => Some(callback.rect),
            _ => None,
        });
    Painted {
        landed: landed_in(&output.shapes),
        viewport,
    }
}

/// **One frame of the real app**, in a `window`-sized window, with
/// `time` and `events` as its input: every text run it painted and
/// where.
#[cfg(test)]
pub(crate) fn app_frame(
    ctx: &egui::Context,
    app: &mut ViewerApp,
    frame: &mut eframe::Frame,
    window: egui::Vec2,
    time: Option<f64>,
    events: Vec<egui::Event>,
) -> Vec<Landed> {
    painted(ctx, app, frame, window, time, events).landed
}

/// Whether the index attempt for the picture on screen is answered: an
/// index held, or its refusal. One that is neither is one `sync` has
/// not asked for yet.
pub(crate) fn index_answered(app: &ViewerApp) -> bool {
    app.picks.index().is_some() || app.picks.error().is_some()
}

/// **Nothing the toolbar's progress read names can change before the
/// next op**: no wait is outstanding (so the document landed and its
/// evaluation is current), and the index attempt is answered.
pub(crate) fn quiescent(app: &ViewerApp) -> bool {
    app.progress().is_none() && index_answered(app)
}

/// **Paint until `until` holds of the app**, and answer the first
/// frame painted in that state, or `None` when [`SETTLE_FRAMES`]
/// frames go by without it.
///
/// The evaluation, the display fit and the index build run on worker
/// threads, so a fixed number of frames catches whichever phase the
/// scheduler reached. Everything `until` reads moves only inside a
/// frame (the session's and the seams' `pump`/`poll` at its top, the
/// cache's `sync`, an op's submit in the batch at its end), so a frame
/// read as settled afterwards painted that state.
fn settled<T>(
    app: &mut ViewerApp,
    until: impl Fn(&ViewerApp) -> bool,
    mut paint: impl FnMut(&mut ViewerApp) -> T,
) -> Option<T> {
    for _ in 0..SETTLE_FRAMES {
        let painted = paint(app);
        if until(app) {
            return Some(painted);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    None
}

/// Why the app has not settled, in the words a stuck run is read in.
fn unsettled(app: &ViewerApp) -> String {
    format!(
        "the app did not settle in {SETTLE_FRAMES} frames: progress {:?}, index held {}, \
         refusal held {}",
        app.progress(),
        app.picks.index().is_some(),
        app.picks.error().is_some(),
    )
}

/// [`settled`], panicking when the app does not settle.
///
/// # Panics
///
/// When [`SETTLE_FRAMES`] frames go by without `until` holding.
#[cfg(test)]
pub(crate) fn settle<T>(
    app: &mut ViewerApp,
    until: impl Fn(&ViewerApp) -> bool,
    paint: impl FnMut(&mut ViewerApp) -> T,
) -> T {
    match settled(app, &until, paint) {
        Some(painted) => painted,
        None => panic!("{}", unsettled(app)),
    }
}

/// Why a driven step could not be taken.
#[derive(Debug)]
pub enum DriveError {
    /// The app could not start.
    Startup(Box<StartupError>),
    /// The label asked for is not on screen as often as asked.
    NotOnScreen {
        /// The label.
        text: String,
        /// Which painting of it (zero-based) was asked for.
        nth: usize,
        /// How many visible paintings of it the last frame had.
        shown: usize,
        /// Visible labels that contain it, ignoring case — what the
        /// caller most likely meant.
        near: Vec<String>,
    },
    /// The viewport pane was not drawn in the last frame.
    NoViewport,
    /// The app did not settle.
    Unsettled(String),
}

impl core::fmt::Display for DriveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Startup(error) => write!(f, "the app could not start: {error}"),
            Self::NotOnScreen {
                text,
                nth,
                shown,
                near,
            } => {
                write!(f, "`{text}` #{nth} is not on screen ({shown} visible)")?;
                if !near.is_empty() {
                    write!(f, "; near: {}", near.join(" | "))?;
                }
                Ok(())
            }
            Self::NoViewport => write!(f, "the viewport was not drawn in the last frame"),
            Self::Unsettled(why) => f.write_str(why),
        }
    }
}

impl std::error::Error for DriveError {}

/// **The real app in a context with no window**, and the pointer,
/// clock and last frame a person at it would have.
pub struct Driver {
    ctx: egui::Context,
    app: ViewerApp,
    frame: eframe::Frame,
    window: egui::Vec2,
    time: f64,
    pointer: egui::Pos2,
    last: Painted,
}

impl Driver {
    /// The startup document at `tol`, settled, in a [`WINDOW`]-sized
    /// window, with preferences read from and written to `prefs` (a path that
    /// does not exist yet reads as no preferences).
    ///
    /// # Errors
    ///
    /// [`DriveError::Startup`] when the app could not start, and
    /// [`DriveError::Unsettled`] when its startup evaluation did not
    /// land.
    pub fn start(prefs: PathBuf, tol: Tol) -> Result<Self, DriveError> {
        let ctx = egui::Context::default();
        let mut app = ViewerApp::assemble_with(&ctx, tol, FileStore::at(prefs))
            .map_err(|error| DriveError::Startup(Box::new(error)))?;
        app.chooser = ChooserBackend::Absent;
        let mut driver = Self {
            ctx,
            app,
            frame: eframe::Frame::_new_kittest(),
            window: WINDOW,
            time: 0.0,
            pointer: egui::Pos2::ZERO,
            last: Painted {
                landed: Vec::new(),
                viewport: None,
            },
        };
        driver.settle()?;
        Ok(driver)
    }

    /// The session the app holds, to read state the screen does not
    /// show.
    #[must_use]
    pub fn session(&self) -> &DocSession {
        &self.app.session
    }

    /// Everything the last frame painted.
    #[must_use]
    pub fn screen(&self) -> &[Landed] {
        &self.last.landed
    }

    /// Where the viewport pane was drawn in the last frame.
    #[must_use]
    pub fn viewport(&self) -> Option<egui::Rect> {
        self.last.viewport
    }

    /// The window the app is laid out in.
    #[must_use]
    pub fn window(&self) -> egui::Vec2 {
        self.window
    }

    /// Lay the app out in a `size` window from the next frame on.
    pub fn resize(&mut self, size: egui::Vec2) {
        self.window = size;
    }

    /// **One frame** with `events` delivered; the clock advances by
    /// one frame first.
    pub fn step(&mut self, events: Vec<egui::Event>) {
        self.time += FRAME_SECONDS;
        self.last = painted(
            &self.ctx,
            &mut self.app,
            &mut self.frame,
            self.window,
            Some(self.time),
            events,
        );
    }

    /// **Frames until the app is quiescent**, then two quiet ones: one
    /// for what landed in the last to be laid out, one for a layout
    /// that sized itself on its first appearance.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`] when it does not settle.
    pub fn settle(&mut self) -> Result<(), DriveError> {
        let Self {
            ctx,
            app,
            frame,
            window,
            time,
            ..
        } = self;
        let last = settled(app, quiescent, |app| {
            *time += FRAME_SECONDS;
            painted(ctx, app, frame, *window, Some(*time), Vec::new())
        })
        .ok_or_else(|| DriveError::Unsettled(unsettled(&self.app)))?;
        self.last = last;
        self.step(Vec::new());
        self.step(Vec::new());
        Ok(())
    }

    /// Where the `nth` (zero-based) visible painting of exactly `text`
    /// is: the centre of its on-screen part.
    ///
    /// # Errors
    ///
    /// [`DriveError::NotOnScreen`] when it is painted visibly fewer
    /// than `nth + 1` times.
    pub fn find(&self, text: &str, nth: usize) -> Result<egui::Pos2, DriveError> {
        let shown: Vec<egui::Rect> = self
            .last
            .landed
            .iter()
            .filter(|landed| landed.text == text)
            .filter_map(Landed::visible)
            .collect();
        shown.get(nth).map(egui::Rect::center).ok_or_else(|| {
            let lower = text.to_lowercase();
            let mut near: Vec<String> = self
                .last
                .landed
                .iter()
                .filter(|landed| landed.visible().is_some())
                .map(|landed| landed.text.clone())
                .filter(|painted| painted.to_lowercase().contains(&lower))
                .collect();
            near.dedup();
            near.truncate(8);
            DriveError::NotOnScreen {
                text: text.to_owned(),
                nth,
                shown: shown.len(),
                near,
            }
        })
    }

    /// Whether `text` is painted visibly at all in the last frame.
    #[must_use]
    pub fn shows(&self, text: &str) -> bool {
        self.find(text, 0).is_ok()
    }

    /// The point `(fx, fy)` of the viewport, as fractions of its width
    /// and height from its top-left corner.
    ///
    /// # Errors
    ///
    /// [`DriveError::NoViewport`] when the viewport was not drawn.
    pub fn in_viewport(&self, fx: f32, fy: f32) -> Result<egui::Pos2, DriveError> {
        let rect = self.last.viewport.ok_or(DriveError::NoViewport)?;
        Ok(rect.lerp_inside(egui::vec2(fx, fy)))
    }

    /// Move the pointer to `at`, in one frame.
    pub fn move_to(&mut self, at: egui::Pos2) {
        self.pointer = at;
        self.step(vec![egui::Event::PointerMoved(at)]);
    }

    /// **Click `button` at `at`** with `modifiers` held, `count` times
    /// in one frame (two is a double click), then settle.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`] when what the click started does not
    /// settle.
    pub fn click_at(
        &mut self,
        at: egui::Pos2,
        button: egui::PointerButton,
        modifiers: egui::Modifiers,
        count: usize,
    ) -> Result<(), DriveError> {
        self.move_to(at);
        let press = |pressed| egui::Event::PointerButton {
            pos: at,
            button,
            pressed,
            modifiers,
        };
        let mut events = Vec::new();
        for _ in 0..count {
            events.push(press(true));
            events.push(press(false));
        }
        self.step(events);
        self.settle()
    }

    /// **Click the `nth` visible painting of `text`** with the primary
    /// button, then settle.
    ///
    /// # Errors
    ///
    /// [`Self::find`]'s, and [`Self::click_at`]'s.
    pub fn click(&mut self, text: &str, nth: usize) -> Result<(), DriveError> {
        let at = self.find(text, nth)?;
        self.click_at(
            at,
            egui::PointerButton::Primary,
            egui::Modifiers::default(),
            1,
        )
    }

    /// **Rest the pointer on the `nth` visible painting of `text`**
    /// until well past the tooltip delay.
    ///
    /// # Errors
    ///
    /// [`Self::find`]'s.
    pub fn hover(&mut self, text: &str, nth: usize) -> Result<(), DriveError> {
        let at = self.find(text, nth)?;
        self.rest_at(at);
        Ok(())
    }

    /// Rest the pointer at `at` until well past the tooltip delay; a
    /// tooltip's area spends the first frame it appears sizing itself,
    /// invisible.
    pub fn rest_at(&mut self, at: egui::Pos2) {
        self.move_to(at);
        let delay = f64::from(self.ctx.global_style().interaction.tooltip_delay);
        let frames = (2.0 * delay / FRAME_SECONDS).ceil() as usize + 3;
        for _ in 0..frames {
            self.step(Vec::new());
        }
    }

    /// **Drag `button` from `from` to `to`** in `steps` moves with
    /// `modifiers` held, then settle.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`] when what the drag started does not
    /// settle.
    pub fn drag(
        &mut self,
        from: egui::Pos2,
        to: egui::Pos2,
        button: egui::PointerButton,
        modifiers: egui::Modifiers,
        steps: usize,
    ) -> Result<(), DriveError> {
        self.move_to(from);
        self.step(vec![egui::Event::PointerButton {
            pos: from,
            button,
            pressed: true,
            modifiers,
        }]);
        let steps = steps.max(1);
        for step in 1..=steps {
            let at = from.lerp(to, step as f32 / steps as f32);
            self.pointer = at;
            self.step(vec![egui::Event::PointerMoved(at)]);
        }
        self.step(vec![egui::Event::PointerButton {
            pos: to,
            button,
            pressed: false,
            modifiers,
        }]);
        self.settle()
    }

    /// **Turn the wheel at `at`** by `delta` points, then settle.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`].
    pub fn scroll(&mut self, at: egui::Pos2, delta: egui::Vec2) -> Result<(), DriveError> {
        self.move_to(at);
        self.step(vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta,
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }]);
        self.settle()
    }

    /// **Type `text`** into whatever has keyboard focus, then settle.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`].
    pub fn type_text(&mut self, text: &str) -> Result<(), DriveError> {
        self.step(vec![egui::Event::Text(text.to_owned())]);
        self.settle()
    }

    /// **Press and release `key`** with `modifiers` held, then settle.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`].
    pub fn key(&mut self, key: egui::Key, modifiers: egui::Modifiers) -> Result<(), DriveError> {
        let event = |pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers,
        };
        self.step(vec![egui::Event::ModifiersChanged(modifiers), event(true)]);
        self.step(vec![
            event(false),
            egui::Event::ModifiersChanged(egui::Modifiers::default()),
        ]);
        self.settle()
    }

    /// **Open the document at `path`**: what Open… ends in once its
    /// dialog has answered.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`].
    pub fn open(&mut self, path: &Path) -> Result<(), DriveError> {
        self.app
            .perform_batch(vec![SessionOp::Open(path.to_path_buf())]);
        self.settle()
    }

    /// **Save the document at `path`**: what Save As… ends in once its
    /// dialog has answered.
    ///
    /// # Errors
    ///
    /// [`DriveError::Unsettled`].
    pub fn save(&mut self, path: &Path) -> Result<(), DriveError> {
        self.app
            .perform_batch(vec![SessionOp::Save(path.to_path_buf())]);
        self.settle()
    }

    /// **The last frame as text**, one visible painting per line in
    /// reading order — top to bottom, then left to right — each as
    /// `x,y wxh text`, with a galley's newlines shown as `⏎`.
    #[must_use]
    pub fn dump(&self) -> String {
        let mut shown: Vec<(egui::Rect, &str)> = self
            .last
            .landed
            .iter()
            .filter_map(|landed| Some((landed.visible()?, landed.text.as_str())))
            .filter(|(_, text)| !text.trim().is_empty())
            .collect();
        shown.sort_by(|(a, _), (b, _)| {
            (a.min.y.round(), a.min.x.round())
                .partial_cmp(&(b.min.y.round(), b.min.x.round()))
                .unwrap_or(core::cmp::Ordering::Equal)
        });
        let mut out = String::new();
        if let Some(rect) = self.last.viewport {
            out.push_str(&format!(
                "[viewport {:.0},{:.0} {:.0}x{:.0}]\n",
                rect.min.x,
                rect.min.y,
                rect.width(),
                rect.height()
            ));
        }
        for (rect, text) in shown {
            out.push_str(&format!(
                "{:.0},{:.0} {:.0}x{:.0} {}\n",
                rect.min.x,
                rect.min.y,
                rect.width(),
                rect.height(),
                text.replace('\n', "⏎")
            ));
        }
        out
    }

    /// **Every painting of the last frame**, clipped or not, in paint
    /// order, each as `x,y wxh [clip x,y wxh] text` with `hidden` on
    /// one whose part on screen is empty — for a control that was laid
    /// out where no one can see it.
    #[must_use]
    pub fn dump_all(&self) -> String {
        let rect_of = |rect: egui::Rect| {
            format!(
                "{:.0},{:.0} {:.0}x{:.0}",
                rect.min.x,
                rect.min.y,
                rect.width(),
                rect.height()
            )
        };
        let mut out = String::new();
        for landed in &self.last.landed {
            if landed.text.trim().is_empty() {
                continue;
            }
            out.push_str(&format!(
                "{} [clip {}]{} {}\n",
                rect_of(landed.allocated),
                rect_of(landed.clip),
                if landed.visible().is_some() {
                    ""
                } else {
                    " hidden"
                },
                landed.text.replace('\n', "⏎")
            ));
        }
        out
    }

    /// The pointer's last position.
    #[must_use]
    pub fn pointer(&self) -> egui::Pos2 {
        self.pointer
    }
}
