//! The eframe application: docked chrome around the wgpu viewport.
//!
//! # What this module is allowed to contain
//!
//! Toolkit adaptation, and nothing else. It turns egui pointer state
//! into [`ViewportEvent`]s, folds those through [`InputMap`] and
//! [`camera::apply`], and hands the resulting camera to the paint
//! callback. Every decision it takes is a call into the renderer-free
//! half of this crate, which is what makes the navigation testable
//! without a window (G1) — and what makes the seam reading in the
//! PR body a statement about egui rather than about our code.
//!
//! # OQ-b: the docking crate is `egui_tiles`
//!
//! The layout is a `Tree<Pane>` the application **owns**: panes are
//! our enum, the tree is our field, and rendering is a `Behavior` impl
//! that reads it. That is the same discipline the rest of this crate
//! runs on, one level up — the layout is a value, the frame is a view
//! of it — and it is the reason the choice went this way rather than
//! to `egui_dock`, whose licence is MIT-only where ours is
//! MIT OR Apache-2.0. The PR body carries the full argument.
//!
//! # No threads
//!
//! Evaluation runs inline, on the frame that asks for it. The plan
//! forbids this layer from assuming threads, and the spike has no
//! evaluation service to put behind a seam yet; the one place a
//! background service will attach is [`ViewerApp::rebuild`], which is
//! already the only function that turns a document into a scene.

use std::sync::Arc;

use eframe::egui;
use egui_tiles::{TileId, Tiles, Tree, UiResponse};
use pncad::document::{Doc, ProfileProgram};
use pncad::geom_core::Tol;

use crate::camera::{self, Camera, CameraOp};
use crate::gpu::{DEPTH_BITS, ViewportCallback, ViewportRenderer};
use crate::input::{InputMap, PointerButton, ViewportEvent, ViewportSize};
use crate::scene::{self, DisplayTolerance, SceneMesh};

/// The window title.
const WINDOW_TITLE: &str = "viewer — GUI-0 scaffold";

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

/// One docked pane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pane {
    /// The 3D viewport.
    Viewport,
    /// The recipe outline — GUI-3's feature tree occupies this seat;
    /// here it lists the document's nodes so the chrome is showing
    /// the real document rather than filler.
    Recipe,
    /// View settings: display δ, the camera's state, and what the
    /// last refused operation was.
    View,
}

/// Everything the application knows.
///
/// The `Doc` is the authoritative value; `scene` and `camera` are
/// derived from it and from navigation, and nothing else is kept.
pub struct ViewerApp {
    document: Doc<ProfileProgram>,
    tol: Tol,
    delta: DisplayTolerance,
    scene: Arc<SceneMesh>,
    /// Bumped on every rebuild; the GPU uploads when it disagrees.
    revision: u64,
    camera: Camera,
    input: InputMap,
    tree: Tree<Pane>,
    /// The last refusal, kept so a rejected operation is visible
    /// instead of silently dropped.
    status: Option<String>,
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
}

impl ViewerApp {
    /// Build the application: author the document, evaluate it,
    /// tessellate at the initial δ, frame a camera on the result, and
    /// install the viewport pipeline into the render state.
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
            document,
            tol,
            delta,
            scene: Arc::new(mesh),
            revision: 1,
            camera,
            input: InputMap::default(),
            tree: initial_layout(),
            status: None,
        })
    }

    /// Re-tessellate at the current δ and re-frame nothing.
    ///
    /// The seam a background evaluation service attaches to: this is
    /// the whole of "the document changed, make a new picture", and it
    /// is called from exactly one place.
    fn rebuild(&mut self) {
        match scene::scene_of(&self.document, self.delta, self.tol) {
            Ok(mesh) => {
                self.scene = Arc::new(mesh);
                self.revision = self.revision.wrapping_add(1);
                self.status = None;
            }
            Err(error) => self.status = Some(format!("scene: {error:?}")),
        }
    }

    /// Apply one camera operation, recording a refusal rather than
    /// dropping it.
    fn navigate(&mut self, op: &CameraOp) {
        match camera::apply(&self.camera, op) {
            Ok(next) => {
                self.camera = next;
                self.status = None;
            }
            Err(error) => self.status = Some(format!("camera: {error:?}")),
        }
    }

    /// Change δ by `factor`, rebuilding the scene.
    fn rescale_delta(&mut self, factor: f64) {
        match self.delta.scaled(factor) {
            Ok(delta) => {
                self.delta = delta;
                self.rebuild();
            }
            Err(error) => self.status = Some(format!("display tolerance: {error:?}")),
        }
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("viewer_toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(WINDOW_TITLE);
                ui.separator();
                if ui.button("Fit").clicked() {
                    let bounds = self.scene.bounds();
                    let aspect = 1.0;
                    self.navigate(&CameraOp::Frame { bounds, aspect });
                }
                if ui.button("Coarser δ").clicked() {
                    self.rescale_delta(DELTA_STEP);
                }
                if ui.button("Finer δ").clicked() {
                    self.rescale_delta(1.0 / DELTA_STEP);
                }
                if let Some(status) = &self.status {
                    ui.separator();
                    ui.label(status.as_str());
                }
            });
        });

        egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut behavior = ViewerBehavior {
                document: &self.document,
                delta: self.delta,
                scene: &self.scene,
                revision: self.revision,
                camera: &mut self.camera,
                input: self.input,
                status: &mut self.status,
            };
            self.tree.ui(&mut behavior, ui);
        });
    }
}

/// The `Behavior` egui_tiles renders panes through: a borrow of the
/// application's state for the duration of one frame.
struct ViewerBehavior<'a> {
    document: &'a Doc<ProfileProgram>,
    delta: DisplayTolerance,
    scene: &'a Arc<SceneMesh>,
    revision: u64,
    camera: &'a mut Camera,
    input: InputMap,
    status: &'a mut Option<String>,
}

impl egui_tiles::Behavior<Pane> for ViewerBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::Viewport => "Viewport".into(),
            Pane::Recipe => "Recipe".into(),
            Pane::View => "View".into(),
        }
    }

    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        match pane {
            Pane::Viewport => self.viewport_ui(ui),
            Pane::Recipe => self.recipe_ui(ui),
            Pane::View => self.view_ui(ui),
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

        let shift = ui.input(|i| i.modifiers.shift);
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
        for event in &events {
            let Some(op) = self.input.map(event, viewport, self.camera) else {
                continue;
            };
            match camera::apply(self.camera, &op) {
                Ok(next) => *self.camera = next,
                Err(error) => *self.status = Some(format!("camera: {error:?}")),
            }
        }

        let matrix = match self.camera.view_projection(aspect) {
            Ok(matrix) => matrix,
            Err(error) => {
                *self.status = Some(format!("projection: {error:?}"));
                return;
            }
        };
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            ViewportCallback {
                scene: Arc::clone(self.scene),
                revision: self.revision,
                view_projection: to_f32(&matrix),
                light_direction: LIGHT_DIRECTION,
                base_color: BASE_COLOR,
            },
        ));
    }

    /// The recipe pane: GUI-3's seat, showing the real document.
    fn recipe_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Recipe");
        ui.label(format!("{} nodes", self.document.order().len()));
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            for id in self.document.order() {
                let label = match self.document.node(*id) {
                    Some(node) => format!("{id:?}: {}", node_kind(node)),
                    None => format!("{id:?}: (absent)"),
                };
                ui.label(label);
            }
        });
        ui.separator();
        ui.label(format!("roots: {}", self.document.roots().len()));
    }

    /// The view pane: the numbers the camera and the tessellation are
    /// actually running at.
    fn view_ui(&mut self, ui: &mut egui::Ui) {
        let stats = self.scene.stats();
        ui.heading("View");
        ui.label(format!("display δ: {:.3} mm", self.delta.get() * 1000.0));
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
        if let Some(status) = self.status.as_ref() {
            ui.separator();
            ui.label(status.as_str());
        }
    }
}

/// A node's kind, for the outline. Deliberately a name and not a
/// rendering of the node: GUI-3 owns what a feature tree shows.
fn node_kind(node: &pncad::document::Node<ProfileProgram>) -> &'static str {
    use pncad::document::Node;
    match node {
        Node::Profile(_) => "Profile",
        Node::Extrude { .. } => "Extrude",
        Node::Revolve { .. } => "Revolve",
        Node::Transform { .. } => "Transform",
        Node::Boolean { .. } => "Boolean",
        Node::Split { .. } => "Split",
        Node::Pattern { .. } => "Pattern",
        Node::PlacedUnion { .. } => "PlacedUnion",
        Node::Datum(_) => "Datum",
        Node::Declare { .. } => "Declare",
        Node::Fillet { .. } => "Fillet",
        Node::Loft { .. } => "Loft",
        Node::Sweep { .. } => "Sweep",
        Node::InstantiatePart { .. } => "InstantiatePart",
        Node::Mate { .. } => "Mate",
    }
}

/// The starting layout: the viewport with a tabbed side panel, in a
/// horizontal split with the viewport taking the larger share.
pub fn initial_layout() -> Tree<Pane> {
    let mut tiles = Tiles::default();
    let viewport = tiles.insert_pane(Pane::Viewport);
    let recipe = tiles.insert_pane(Pane::Recipe);
    let view = tiles.insert_pane(Pane::View);
    let side = tiles.insert_tab_tile(vec![recipe, view]);
    // Three quarters of the width to the viewport: the side panel is
    // GUI-3's seat, not this unit's subject.
    let linear =
        egui_tiles::Linear::new_binary(egui_tiles::LinearDir::Horizontal, [viewport, side], 0.75);
    let root = tiles.insert_container(egui_tiles::Container::Linear(linear));
    Tree::new("viewer_tree", root, tiles)
}

/// Column-major `f64` matrix to the `f32` the GPU consumes.
fn to_f32(matrix: &[[f64; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0f32; 4]; 4];
    for (col, values) in matrix.iter().enumerate() {
        for (row, value) in values.iter().enumerate() {
            if let Some(slot) = out.get_mut(col).and_then(|c| c.get_mut(row)) {
                *slot = *value as f32;
            }
        }
    }
    out
}

/// Run the application.
///
/// The depth buffer request is load-bearing — see `gpu`'s module docs.
///
/// # Errors
///
/// `eframe`'s own startup error, or a [`StartupError`] boxed into it:
/// a viewer that cannot build its scene reports why and exits rather
/// than opening a window onto nothing.
pub fn run(tol: Tol) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        depth_buffer: DEPTH_BITS,
        ..Default::default()
    };
    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(move |cc| {
            let app = ViewerApp::new(cc, tol).map_err(|error| format!("{error:?}"))?;
            Ok(Box::new(app))
        }),
    )
}
