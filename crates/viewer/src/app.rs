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

use std::sync::Arc;

use eframe::egui;
use egui_tiles::{TileId, Tiles, Tree, UiResponse};
use pncad::document::{Dimension, RecipeNodeId, SlotId};
use pncad::geom_core::Tol;

use crate::camera::{self, Camera, CameraOp};
use crate::evalseam::{Generation, ThreadEvaluator};
use crate::gpu::{DEPTH_BITS, ViewportCallback, ViewportRenderer};
use crate::input::{self, InputMap, PointerButton, ViewportEvent, ViewportSize};
use crate::props::{SlotDriver, SlotRow, SlotValue};
use crate::scene::{self, DisplayTolerance, SceneMesh};
use crate::session::{DocSession, Refusal, Selection, SessionOp};
use crate::tree::{RowStatus, TreeRow};

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
const DOC_EXTENSION: &str = "pncad";

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
}

/// Everything the application knows.
pub struct ViewerApp {
    session: DocSession,
    tol: Tol,
    delta: DisplayTolerance,
    scene: Arc<SceneMesh>,
    /// Bumped on every rebuild; the GPU uploads when it disagrees.
    revision: u64,
    /// The evaluation generation `scene` was built from. When it
    /// disagrees with the session's landed generation, the picture is
    /// out of date and exactly one rebuild is owed.
    scene_generation: Option<Generation>,
    camera: Camera,
    input: InputMap,
    tree: Tree<Pane>,
    drafts: Drafts,
    /// A fit is owed, and will be taken by the viewport pane on the
    /// next frame — the only place that knows the real aspect.
    pending_fit: bool,
    /// The last thing that went wrong, kept so a refused operation is
    /// visible instead of silently dropped.
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
    /// The evaluation worker could not be started. Fatal on purpose: a
    /// seam with no worker accepts every submit and answers none, so
    /// the application would open onto a permanent "evaluating…".
    Evaluator(crate::evalseam::SpawnError),
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
            session: DocSession::new(
                document,
                tol,
                Box::new(ThreadEvaluator::spawn().map_err(StartupError::Evaluator)?),
            ),
            tol,
            delta,
            scene: Arc::new(mesh),
            revision: 1,
            scene_generation: None,
            camera,
            input: InputMap::default(),
            tree: initial_layout(),
            drafts: Drafts::default(),
            pending_fit: true,
            status: None,
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
        if self.scene_generation == self.session.landed_generation() {
            return;
        }
        let Some(evaluation) = self.session.evaluation_arc().map(Arc::clone) else {
            return;
        };
        self.scene_generation = self.session.landed_generation();
        match scene::scene_of_evaluation(self.session.doc(), &evaluation, self.delta, self.tol) {
            Ok(mesh) => {
                self.scene = Arc::new(mesh);
                self.revision = self.revision.wrapping_add(1);
                self.status = None;
            }
            Err(error) => self.status = Some(format!("scene: {error:?}")),
        }
    }

    /// Change δ, rebuilding the picture from the evaluation already in
    /// hand — a display change is not a document change and re-runs no
    /// geometry above the tessellator.
    fn rescale_delta(&mut self, factor: f64) {
        match self.delta.scaled(factor) {
            Ok(delta) => {
                self.delta = delta;
                self.scene_generation = None;
                self.sync_scene();
            }
            Err(error) => self.status = Some(format!("display tolerance: {error:?}")),
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
        for op in ops {
            if let Some(next) = self.session.perform(op).refusal {
                refusal = Refusal::preferred(refusal, next);
            }
        }
        self.status = refusal.map(|refusal| refusal.to_string());
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
                if ui.button("Open…").clicked()
                    && let Some(path) = pick_open()
                {
                    ops.push(SessionOp::Open(path));
                }
                if ui.button("Save As…").clicked()
                    && let Some(path) = pick_save(self.session.path())
                {
                    ops.push(SessionOp::Save(path));
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
                if let Some(status) = &self.status {
                    ui.separator();
                    ui.label(status.as_str());
                }
            });
        });

        egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut behavior = ViewerBehavior {
                session: &self.session,
                delta: self.delta,
                scene: &self.scene,
                revision: self.revision,
                camera: &mut self.camera,
                input: self.input,
                drafts: &mut self.drafts,
                pending_fit: &mut self.pending_fit,
                status: &mut self.status,
                ops: &mut ops,
            };
            self.tree.ui(&mut behavior, ui);
        });

        self.perform_batch(ops);
    }
}

/// The `Behavior` egui_tiles renders panes through: a borrow of the
/// application's state for the duration of one frame.
struct ViewerBehavior<'a> {
    session: &'a DocSession,
    delta: DisplayTolerance,
    scene: &'a Arc<SceneMesh>,
    revision: u64,
    camera: &'a mut Camera,
    input: InputMap,
    drafts: &'a mut Drafts,
    pending_fit: &'a mut bool,
    status: &'a mut Option<String>,
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
        .map(|(op, error)| format!("camera: {error:?} (from {op:?})"));
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

    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        match pane {
            Pane::Viewport => self.viewport_ui(ui),
            Pane::Features => self.features_ui(ui),
            Pane::Properties => self.properties_ui(ui),
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
        let folded = input::fold_events(&self.input, self.camera, viewport, &events);
        if !events.is_empty() {
            land(self.camera, self.status, &folded);
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

    /// The feature tree: one row per recipe node, with its status
    /// badge from the evaluation's typed result.
    fn features_ui(&mut self, ui: &mut egui::Ui) {
        let rows = self.session.tree_rows();
        ui.horizontal(|ui| {
            ui.heading("Features");
            ui.label(format!("{} nodes", rows.len()));
        });
        ui.separator();
        let selected = match self.session.selection() {
            Selection::Node(id) => Some(*id),
            Selection::None | Selection::Param(_) => None,
        };
        egui::ScrollArea::vertical().show(ui, |ui| {
            for row in &rows {
                self.feature_row(ui, row, selected == Some(row.id));
            }
        });
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
            match &row.status {
                RowStatus::Ok => {}
                RowStatus::Unevaluated => {
                    ui.weak(row.status.badge());
                }
                RowStatus::Failed { .. } | RowStatus::Poisoned { .. } => {
                    ui.colored_label(egui::Color32::from_rgb(210, 90, 70), row.status.badge());
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
    }

    /// The property panel.
    fn properties_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Properties");
        ui.separator();
        match self.session.selection().clone() {
            Selection::None => {
                ui.weak("select a feature");
            }
            Selection::Node(node) => {
                let rows = self.session.slot_rows();
                if rows.is_empty() {
                    ui.weak("this feature carries no parameters");
                }
                for row in &rows {
                    self.slot_ui(ui, node, row);
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
                        |value| SessionOp::SetParam {
                            name: name.clone(),
                            value: SlotValue::of(row.dimension, value),
                        },
                        self.ops,
                    );
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
    }

    /// One slot row, with the expression-driven refusal's affordance
    /// attached to it.
    fn slot_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        ui.horizontal(|ui| {
            ui.label(format!("{:?}", row.slot));
            match row.value {
                Ok(value) => {
                    let mut number = value.as_f64();
                    let widget =
                        ui.add(egui::DragValue::new(&mut number).speed(if row.structural {
                            1.0
                        } else {
                            0.0005
                        }));
                    drag_ops(
                        &widget,
                        number,
                        SessionOp::BeginGesture {
                            node,
                            slot: row.slot,
                        },
                        |number| SessionOp::SetSlot {
                            node,
                            slot: row.slot,
                            value: SlotValue::of(row.dimension, number),
                        },
                        self.ops,
                    );
                }
                Err(ref error) => {
                    ui.weak(format!("{error}"));
                }
            }
            ui.weak(format!("{:?}", row.dimension));
            if row.structural {
                ui.weak("structural");
            }
        });
        // The affordance. It is attached to the row rather than raised
        // on refusal alone so the user can see WHY the number will not
        // move before they fight it — the refusal itself still
        // surfaces in the status line when they try.
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
        // The expression text door, offered on every slot: the way to
        // replace a computation is to write a new one.
        let target = (node, row.slot);
        if self.drafts.expr_target == Some(target) {
            ui.horizontal(|ui| {
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
        } else if ui.small_button("expression…").clicked() {
            // Deliberately EMPTY rather than pre-filled: the
            // expression API has no text rendering, so a pre-filled
            // field would be this crate's guess at what the slot says.
            // See the module docs of `props`.
            self.drafts.expr_target = Some(target);
            self.drafts.expr_text.clear();
        }
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
fn drag_ops(
    widget: &egui::Response,
    value: f64,
    begin: SessionOp,
    typed: impl Fn(f64) -> SessionOp,
    ops: &mut Vec<SessionOp>,
) {
    if widget.drag_started() {
        ops.push(begin);
    }
    if widget.dragged() && widget.changed() {
        ops.push(SessionOp::PreviewGesture { value });
    }
    if widget.drag_stopped() {
        ops.push(SessionOp::CommitGesture);
    } else if widget.changed() && !widget.dragged() {
        // Typed, not dragged: one edit, no gesture.
        ops.push(typed(value));
    }
}

/// The open dialog: a THIN veneer over `SessionOp::Open`.
///
/// Everything it does is choose a `Path`. The blocking call is
/// deliberate — a modal file chooser is the platform's own idea of a
/// modal file chooser, and the alternative (an async handle polled
/// across frames) would buy responsiveness during an interaction that
/// is already modal, at the cost of a second state machine.
fn pick_open() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter("document", &[DOC_EXTENSION])
        .pick_file()
}

/// The save dialog, starting where the current document lives.
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
