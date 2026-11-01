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
//! operations and ray picks, and the paint callback it queues is run
//! only when [`Driver::picture`] is asked for a picture of the frame,
//! on a device the caller hands it.
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

use eframe::{egui, egui_wgpu, wgpu};
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

/// What one frame of the real app showed: its text, the rect of the
/// viewport's paint callback when the viewport was drawn, and the
/// shapes and texture changes a picture of it is rendered from.
struct Painted {
    landed: Vec<Landed>,
    viewport: Option<egui::Rect>,
    shapes: Vec<egui::epaint::ClippedShape>,
    textures: egui::TexturesDelta,
}

impl Painted {
    fn nothing() -> Self {
        Self {
            landed: Vec::new(),
            viewport: None,
            shapes: Vec::new(),
            textures: egui::TexturesDelta::default(),
        }
    }
}

/// **The textures every frame so far has set and not freed**, as the
/// image deltas that set them, oldest first. A picture uploads them to
/// a renderer of its own: egui sends the font atlas once, on the first
/// frame, and only its changes after that.
#[derive(Default)]
struct Textures(std::collections::HashMap<egui::TextureId, Vec<egui::epaint::ImageDelta>>);

impl Textures {
    /// Take one frame's changes in.
    fn fold(&mut self, mut delta: egui::TexturesDelta) {
        for (id, images) in std::mem::take(&mut delta.set) {
            for image in images {
                let held = self.0.entry(id).or_default();
                if image.is_whole() {
                    held.clear();
                }
                held.push(image);
            }
        }
        for id in std::mem::take(&mut delta.free) {
            self.0.remove(&id);
        }
    }
}

/// **A picture of the window**: `size` pixels, row-major from the top
/// left, four bytes per pixel in RGBA order, gamma-encoded as the
/// window shows them.
#[derive(Clone, Debug)]
pub struct Picture {
    /// Width and height in pixels.
    pub size: [u32; 2],
    /// The pixels.
    pub rgba: Vec<u8>,
}

/// **The device the running app gets**, on whatever adapter this
/// machine has: the device descriptor is `egui_wgpu`'s own, the one
/// `NativeOptions`' default setup requests. A future, so the caller
/// brings the executor.
///
/// # Errors
///
/// [`DriveError::Picture`] when there is no adapter or it yields no
/// device. A box with no GPU can install a software one:
/// `mesa-vulkan-drivers` supplies lavapipe, which needs no display.
pub async fn picture_device() -> Result<(wgpu::Device, wgpu::Queue), DriveError> {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .map_err(|error| {
            DriveError::Picture(format!(
                "no wgpu adapter ({error}); `mesa-vulkan-drivers` supplies a software one"
            ))
        })?;
    let egui_wgpu::WgpuSetup::CreateNew(setup) = egui_wgpu::WgpuConfiguration::default().wgpu_setup
    else {
        return Err(DriveError::Picture(
            "egui_wgpu's default setup is no longer `CreateNew`".to_owned(),
        ));
    };
    adapter
        .request_device(&(setup.device_descriptor)(&adapter))
        .await
        .map_err(|error| DriveError::Picture(format!("the adapter yields no device: {error}")))
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
    let output = ctx.run_ui(input, |ui| {
        eframe::App::ui(app, ui, frame);
    });
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
        shapes: output.shapes,
        textures: output.textures_delta,
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
    let mut painted = painted(ctx, app, frame, window, time, events);
    // Nothing here paints, so the frame's texture delta is dropped
    // rather than uploaded; `TexturesDelta` asserts on drop until it
    // is emptied.
    painted.textures.clear();
    painted.landed
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

/// **[`quiescent`], and no check is still running at rest** — the
/// toolbar's checks badge says neither "pending" nor "more pending"
/// ([`crate::frame::checks_badge`]). The coincidence door runs on a
/// worker after the evaluation lands, so a driven step read only to
/// [`quiescent`] can see the findings before the door has answered.
/// A staged run ([`crate::session::SessionOp::Stage`]) is waited for
/// too: the panel that asked says "checking" until it answers.
/// The in-crate rows wait on [`quiescent`] alone: a row about the
/// checks waits for them itself.
fn at_rest(app: &ViewerApp) -> bool {
    quiescent(app)
        && app
            .session
            .checks()
            .is_none_or(|report| report.pending.is_empty())
        && !app.session.staging()
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
         refusal held {}, checks pending {:?}",
        app.progress(),
        app.picks.index().is_some(),
        app.picks.error().is_some(),
        app.session.checks().map(|report| &report.pending),
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
    /// No picture could be rendered: no device, or its readback failed.
    Picture(String),
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
            Self::Unsettled(why) | Self::Picture(why) => f.write_str(why),
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
    textures: Textures,
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
            last: Painted::nothing(),
            textures: Textures::default(),
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

    /// The picture the app holds, drawn or not.
    #[must_use]
    pub fn scene(&self) -> &crate::scene::SceneMesh {
        &self.app.scene
    }

    /// The pick index the app holds, if one is built for the landed run.
    #[must_use]
    pub fn index(&self) -> Option<&crate::pickindex::PickIndex> {
        self.app.picks.index()
    }

    /// **The split plane the viewport draws this frame**
    /// ([`crate::splitplane::drawn`], over the app's own tools, index and
    /// the split panel's numbers).
    #[must_use]
    pub fn split_plane(&self) -> Option<crate::splitplane::PlanePreview> {
        crate::splitplane::drawn(
            &self.app.tools,
            &self.app.session,
            self.app.picks.index(),
            self.app.drafts.split_origin,
            self.app.drafts.split_normal,
        )
    }

    /// The split panel's plane point, as the panel holds it.
    #[must_use]
    pub fn split_origin(&self) -> [f64; 3] {
        self.app.drafts.split_origin
    }

    /// The pick-index seam's badge, as the toolbar draws it.
    #[must_use]
    pub fn index_badge(&self) -> Option<crate::frame::Badge> {
        crate::frame::index_badge(
            self.app.picks.error(),
            self.app.picks.index(),
            self.app.session.landed_pair(),
        )
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
        let mut last = painted(
            &self.ctx,
            &mut self.app,
            &mut self.frame,
            self.window,
            Some(self.time),
            events,
        );
        self.textures.fold(std::mem::take(&mut last.textures));
        self.last = last;
    }

    /// **A picture of the window as the last frame painted it** —
    /// the chrome and the viewport both, rendered offscreen on `device`
    /// the way the window's painter renders a frame: egui's renderer
    /// over the frame's own shapes, with the viewport's paint callback
    /// run through the app's [`crate::gpu::ViewportRenderer`] at the
    /// app's depth format. [`picture_device`] opens a device to hand
    /// it.
    ///
    /// The frame's paint callback is run as the window runs it, its
    /// GPU id query included, so the frame after a picture reads that
    /// query's answer as the window's next frame would.
    ///
    /// # Errors
    ///
    /// [`DriveError::Picture`] when the window has no pixels or the
    /// readback fails.
    pub fn picture(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Picture, DriveError> {
        use crate::gpu::{COPY_ROW_ALIGNMENT, DEPTH_FORMAT, ViewportRenderer};
        const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
        let pixels_per_point = self.ctx.pixels_per_point();
        let pixels = |points: f32| {
            let pixels = (points * pixels_per_point).round();
            (1.0..=16384.0)
                .contains(&pixels)
                .then(|| u32::try_from(pixels as i64).ok())
                .flatten()
                .ok_or_else(|| {
                    DriveError::Picture(format!("a window side of {points} points has no pixels"))
                })
        };
        let size = [pixels(self.window.x)?, pixels(self.window.y)?];
        let mut renderer = egui_wgpu::Renderer::new(
            device,
            FORMAT,
            egui_wgpu::RendererOptions {
                depth_stencil_format: Some(DEPTH_FORMAT),
                ..egui_wgpu::RendererOptions::PREDICTABLE
            },
        );
        renderer
            .callback_resources
            .insert(ViewportRenderer::new(device, FORMAT));
        for (id, images) in &self.textures.0 {
            for image in images {
                renderer.update_texture(device, queue, *id, image);
            }
        }
        let primitives = self
            .ctx
            .tessellate(self.last.shapes.clone(), pixels_per_point);
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point,
        };
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("drive picture"),
        });
        let prepared = renderer.update_buffers(device, queue, &mut encoder, &primitives, &screen);
        let extent = wgpu::Extent3d {
            width: size[0],
            height: size[1],
            depth_or_array_layers: 1,
        };
        let target = |format, usage| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("drive picture"),
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                view_formats: &[],
            })
        };
        let colour = target(
            FORMAT,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        );
        let depth = target(DEPTH_FORMAT, wgpu::TextureUsages::RENDER_ATTACHMENT);
        {
            let colour_view = colour.create_view(&wgpu::TextureViewDescriptor::default());
            let depth_view = depth.create_view(&wgpu::TextureViewDescriptor::default());
            // Cleared as the window's painter clears: depth to 1.0,
            // colour to black under the panels that cover it.
            let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("drive picture"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &colour_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            renderer.render(&mut pass.forget_lifetime(), &primitives, &screen);
        }
        let row = u64::from(size[0]) * 4;
        let padded = row.div_ceil(COPY_ROW_ALIGNMENT) * COPY_ROW_ALIGNMENT;
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("drive picture"),
            size: padded * u64::from(size[1]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let fault = |what: &str, error: &dyn std::fmt::Display| {
            DriveError::Picture(format!("the picture's {what} failed: {error}"))
        };
        let padded_u32 = u32::try_from(padded).map_err(|error| fault("row size", &error))?;
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &colour,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_u32),
                    rows_per_image: Some(size[1]),
                },
            },
            extent,
        );
        queue.submit(
            prepared
                .into_iter()
                .chain(std::iter::once(encoder.finish())),
        );
        let (sent, mapped) = std::sync::mpsc::channel();
        readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                // The receiver is held below until this has answered.
                let _ = sent.send(result);
            });
        device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|error| fault("poll", &error))?;
        mapped
            .recv()
            .map_err(|error| fault("map", &error))?
            .map_err(|error| fault("map", &error))?;
        let view = readback
            .slice(..)
            .get_mapped_range()
            .map_err(|error| fault("mapped range", &error))?;
        let row = usize::try_from(row).map_err(|error| fault("row size", &error))?;
        let padded = usize::try_from(padded).map_err(|error| fault("row size", &error))?;
        let rgba = view
            .chunks_exact(padded)
            .flat_map(|line| &line[..row])
            .copied()
            .collect();
        Ok(Picture { size, rgba })
    }

    /// **Frames until the app is at rest** (quiescent, and no check
    /// still running), then two quiet ones: one for what landed in the
    /// last to be laid out, one for a layout that sized itself on its
    /// first appearance.
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
            textures,
            ..
        } = self;
        let last = settled(app, at_rest, |app| {
            *time += FRAME_SECONDS;
            let mut painted = painted(ctx, app, frame, *window, Some(*time), Vec::new());
            textures.fold(std::mem::take(&mut painted.textures));
            painted
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
