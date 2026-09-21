//! The creation forms: the modal tools and the add-a-node panels the
//! Properties pane hosts above the selection.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{AxisSense, BooleanOp, DocumentId, MatePrimitive, RecipeNodeId};

use crate::app::ViewerBehavior;
use crate::blend::{BlendError, BlendKindChoice, BlendTarget, FREEZE_NOTE};
use crate::combine::PatternOutputChoice;
use crate::drafts::{CommitFault, Drafts, scalars};
use crate::forms::{
    ANGLE_DRAG_SPEED, COUNT_DRAG_SPEED, DatumKindChoice, FIELD_DRAG_SPEED, MATE_PRIMITIVES,
    PatternKindChoice, ShapeEdits, ShapeKind, UNIT_DRAG_SPEED, boolean_op_label,
};
use crate::frame;
use crate::matetool::{MateChoice, MateToolState, admitted_classes};
use crate::pane::profile::{notation_row, path_steps_ui, preview_verdict};
use crate::parts::PartChooser;
use crate::seats::{Seat, seat_line};
use crate::session::{FaceFrameFault, Selection, SessionOp, face_frame_seat};
use crate::sketch;
use crate::tools::ToolKind;
use crate::widgets::{
    angle_picker, length_picker, number_field, point_fields, unit_field, unit_vec3_row, vec3_row,
};

/// **The smallest pattern count the form offers.**
///
/// A pattern of zero instances refuses at evaluation
/// (`NonPositiveCount`, typed, on the node's own badge), so the form
/// declines to author one — the same rule the bore field follows. There
/// is deliberately NO upper bound: the property panel imposes none on
/// the slot afterwards, and a cap here would be a limit the document
/// does not have.
pub(crate) const MIN_PATTERN_COUNT: i64 = 1;

/// **A form's frame pick**: a combo over the document's frames, or a
/// line saying there are none. One widget for every form that writes
/// against a frame, so they name frames one way.
fn frame_picker(
    ui: &mut egui::Ui,
    label: &str,
    salt: &str,
    frames: &[RecipeNodeId],
    picked: &mut Option<RecipeNodeId>,
) {
    ui.horizontal(|ui| {
        ui.label(label);
        if frames.is_empty() {
            ui.weak("none in this document — add a frame datum first");
        } else {
            let shown =
                picked.map_or_else(|| "pick one".to_owned(), |id| format!("feature {}", id.0));
            egui::ComboBox::from_id_salt(salt)
                .selected_text(shown)
                .show_ui(ui, |ui| {
                    for id in frames {
                        ui.selectable_value(picked, Some(*id), format!("feature {}", id.0));
                    }
                });
        }
    });
}

impl ViewerBehavior<'_> {
    /// The creation section (GAUTH-1): the add-datum, add-profile and
    /// extrude forms plus the modal revolve tool. Each form is
    /// minimal — its few required fields with sensible defaults — and
    /// emits exactly one creation op; the property panel is the
    /// editor for everything after the insert — for a profile, through
    /// this section's own path editor opened on the node
    /// ([`Self::edit_profile_ui`]).
    pub(crate) fn create_ui(&mut self, ui: &mut egui::Ui) {
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

    /// The mate tool's panel: activation, the held picks, the class
    /// choice with the kernel's admission verdicts, and the one
    /// committed edit.
    pub(crate) fn mate_tool_ui(&mut self, ui: &mut egui::Ui) {
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
                ui.weak(format!("pick a: face of node {}", a.node.0));
            }
            MateToolState::Two { a, b } => {
                ui.weak(format!(
                    "pick a: node {}; pick b: node {}",
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
                        match tool.proposal(
                            doc,
                            eval,
                            &self.session.eval_options(),
                            self.session.tol(),
                            choice,
                        ) {
                            Ok(proposal) => {
                                // Exactly one committed DocEdit; the
                                // tool closes with it.
                                self.ops.push(proposal.op());
                                close = true;
                            }
                            Err(error) => {
                                self.notices
                                    .push(frame::tool_news(ToolKind::Mate.says(&error)));
                            }
                        }
                    }
                    _ => {
                        self.notices.push(frame::tool_news(
                            ToolKind::Mate.says(&"no landed evaluation to derive frames from"),
                        ));
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
    pub(crate) fn add_part_ui(&mut self, ui: &mut egui::Ui) {
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
                *self.part_chooser = Some(PartChooser::opened(self.session.part_census()));
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
            chooser.rescan(self.session.part_census());
        }
        if close {
            *self.part_chooser = None;
        }
        ui.separator();
    }

    /// The add-datum form: one kind choice, the kind's fields, one
    /// [`SessionOp::AddDatum`] on commit.
    ///
    /// Every kind but one is numbers alone. An axis in a sketch also
    /// names the frame its coordinates are written in, picked from the
    /// document's frames the way the add-profile form picks its plane,
    /// and the button waits until one is picked.
    pub(crate) fn add_datum_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("datum");
            for (kind, label) in DatumKindChoice::ALL {
                ui.radio_value(&mut self.drafts.datum_kind, kind, label);
            }
        });
        let kind = self.drafts.datum_kind;
        match kind {
            DatumKindChoice::Plane => {
                self.datum_origin_row(ui, "origin");
                vec3_row(
                    ui,
                    "normal",
                    UNIT_DRAG_SPEED,
                    &mut self.drafts.datum_direction,
                );
            }
            DatumKindChoice::Frame => {
                self.datum_origin_row(ui, "origin");
                vec3_row(ui, "x axis", UNIT_DRAG_SPEED, &mut self.drafts.datum_u);
                vec3_row(ui, "y axis", UNIT_DRAG_SPEED, &mut self.drafts.datum_v);
                // What the form does to the y axis before it becomes a
                // datum, said where it is being typed: a reader who
                // enters a y that is not square to x gets a frame that
                // is, and a silent correction is the kind a person
                // discovers by measuring the model.
                ui.label("y is squared against x; the normal is x × y");
            }
            DatumKindChoice::Axis => {
                self.datum_origin_row(ui, "origin");
                vec3_row(
                    ui,
                    "direction",
                    UNIT_DRAG_SPEED,
                    &mut self.drafts.datum_direction,
                );
            }
            DatumKindChoice::AxisInPlane => {
                let frames = self.frames();
                frame_picker(
                    ui,
                    "in frame",
                    "datum_frame",
                    &frames,
                    &mut self.drafts.datum_frame,
                );
                let unit = self.drafts.length_unit.def();
                ui.horizontal(|ui| {
                    ui.label("origin");
                    point_fields(ui, unit, &mut self.drafts.datum_in_frame_origin);
                    length_picker(ui, "datum_origin", &mut self.drafts.length_unit);
                });
                ui.horizontal(|ui| {
                    ui.label("direction");
                    for (axis, component) in ["x", "y"]
                        .into_iter()
                        .zip(&mut self.drafts.datum_in_frame_direction)
                    {
                        ui.label(axis);
                        ui.add(number_field(component, UNIT_DRAG_SPEED));
                    }
                });
                // The same-frame rule is the revolve's, and a person
                // authoring this axis is about to meet it: say it here
                // rather than at the revolve's refusal.
                ui.label("x and y are the frame's own; a revolve needs its profile on this frame");
            }
            DatumKindChoice::FaceFrame => self.datum_face_frame_rows(ui),
            DatumKindChoice::Point => self.datum_origin_row(ui, "position"),
        }
        // Lowered every frame, so the button's enabling and its commit
        // read one value: `Ok(None)` is a pick still missing, and it is
        // what holds the button. The sentence over it follows the KIND
        // — the two picking kinds want different things from different
        // places, so one sentence for the form would be false of
        // whichever is not showing.
        let datum = self.drafts.datum_spec();
        let unpicked = matches!(datum, Ok(None));
        if unpicked && let Some(wanted) = kind.unmet_seat() {
            ui.weak(wanted);
        }
        // The face-frame gate, on the kind that has one: a refusal
        // holds the button too, so a frame is not minted on a face the
        // node would refuse at evaluation.
        let refused = (kind == DatumKindChoice::FaceFrame)
            .then(|| face_frame_seat(self.session.landed_pair(), self.drafts.datum_face.as_ref()))
            .and_then(Result::err);
        // `NoFace` is the unmet seat above, said once: the sentence
        // over the button already asks for the pick.
        if let Some(fault) = &refused
            && *fault != FaceFrameFault::NoFace
        {
            ui.weak(fault.to_string());
        }
        if ui
            .add_enabled(
                !unpicked && refused.is_none(),
                egui::Button::new("Add datum"),
            )
            .clicked()
        {
            match datum {
                Ok(Some(datum)) => self.ops.push(SessionOp::AddDatum { datum }),
                Ok(None) => {}
                // The add-datum form is not a seated TOOL, so it has
                // no `ToolKind` to compose the prefix — the form's own
                // name is the sentence's subject here.
                Err(error) => {
                    self.notices
                        .push(frame::tool_news(format!("add datum: {error}")));
                }
            }
        }
    }

    /// **The frame-on-face form's rows**: the held face pick and the
    /// spin.
    ///
    /// The seat LATCHES the viewport's face pick — a face is not a
    /// node a combo can list, so the selection is this form's picker
    /// and there is no widget to draw for it. Writing it here rather
    /// than reading the live selection at the commit is what lets an
    /// author pick a face, type a spin, open the unit picker and click
    /// the feature tree without losing the pick; a second face pick
    /// moves the seat, and nothing else clears it.
    ///
    /// Only the picks are decided here. Whether the face may carry a
    /// frame at all is [`face_frame_seat`]'s answer, rendered by the
    /// caller over the button it holds.
    fn datum_face_frame_rows(&mut self, ui: &mut egui::Ui) {
        if let Selection::Face(face) = self.session.selection() {
            self.drafts.datum_face = Some(face.clone());
        }
        ui.horizontal(|ui| {
            ui.label("face");
            match &self.drafts.datum_face {
                Some(face) => ui.weak(format!("feature {} body {}", face.node.0, face.body)),
                None => ui.weak("none picked"),
            };
        });
        ui.horizontal(|ui| {
            ui.label("spin");
            unit_field(
                ui,
                self.drafts.angle_unit.def(),
                ANGLE_DRAG_SPEED,
                &mut self.drafts.datum_spin,
            );
            angle_picker(ui, "datum_spin", &mut self.drafts.angle_unit);
        });
        // What the number MEANS, said where it is typed: the origin
        // and the normal are the face's, so this rotation is the whole
        // of what an author chooses here.
        ui.label("sketch +x, turned about the face's outward normal; the origin is the face's");
    }

    /// The add-datum form's 3-D Length row — an origin or a position —
    /// with the form's unit picker beside it.
    fn datum_origin_row(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.horizontal(|ui| {
            unit_vec3_row(
                ui,
                label,
                self.drafts.length_unit.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.datum_origin,
            );
            length_picker(ui, "datum_origin", &mut self.drafts.length_unit);
        });
    }

    /// **Every frame the landed document holds**, in document order —
    /// what a form's frame picker offers. Empty with no landed
    /// document, which the picker says rather than hides.
    fn frames(&self) -> Vec<RecipeNodeId> {
        self.session
            .landed_pair()
            .map(|(doc, _)| sketch::frames(doc))
            .unwrap_or_default()
    }

    /// The add-profile form: a template shape with Length fields, one
    /// [`SessionOp::AddProfile`] on commit — on a frame picked from the
    /// document's frames.
    ///
    /// The circle's optional bore is what lets this template author
    /// the hollow ring's annulus (one profile node, two loops).
    ///
    /// **The plane is a PICK of a frame that exists**, and the picker
    /// lists both frame kinds, so drawing on a picked FACE is minting
    /// that face's frame in the add-datum form
    /// ([`DatumKindChoice::FaceFrame`]) and choosing it here. Doing
    /// both in one gesture, and naming a frame in this picker by the
    /// face it sits on rather than by its feature number, is the
    /// residue `work/author/add-profile-mints-no-frame.md` carries.
    ///
    /// The bore field is guarded IN THE FORM: loop roles come from
    /// the profile layer's containment forest, not from list order,
    /// so a bore at or beyond the outer radius would not refuse — it
    /// would silently swap which circle is the hole. The form says
    /// "bore", so it disables Create until the bore is smaller, with
    /// the reason shown. This is a chrome affordance guarding the
    /// template's stated intent; the op stays unjudged and the
    /// kernel's containment rule stays the one home.
    pub(crate) fn add_profile_ui(&mut self, ui: &mut egui::Ui) {
        self.profile_drawn.create = true;
        ui.horizontal(|ui| {
            ui.label("profile");
            for (shape, label) in ShapeKind::ALL {
                ui.radio_value(&mut self.drafts.profile_shape, Some(shape), label);
            }
        });
        // **The frame it is drawn on**, picked from the ones the
        // document holds. A profile's plane is a node, so the form names
        // one — and a document with no frame in it says so rather than
        // conjuring one.
        let frames = self.frames();
        frame_picker(
            ui,
            "on frame",
            "profile_plane",
            &frames,
            &mut self.drafts.profile_plane,
        );
        let shape = self.drafts.profile_shape;
        let mut blocked: Option<&'static str> = None;
        // Stated before the shape check so the FIRST thing a person is
        // told is the thing they have to do first.
        if self.drafts.profile_plane.is_none() {
            blocked = Some("pick a frame to draw on");
        }
        match shape {
            // No shape chosen: the form is at rest. It says what it is
            // waiting for and draws nothing — no fields to fill in for
            // a shape nobody picked, and no preview in the viewport.
            None => blocked = blocked.or(Some("choose a shape to add")),
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
                notation_row(
                    ui,
                    "path",
                    &mut self.drafts.length_unit,
                    &mut self.drafts.angle_unit,
                );
                path_steps_ui(
                    ui,
                    "path",
                    self.session.tol(),
                    (self.drafts.length_unit.def(), self.drafts.angle_unit.def()),
                    ShapeEdits::Free,
                    &mut self.drafts.profile_path,
                );
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
        let refused = preview_verdict(
            ui,
            self.theme,
            self.profile_previews.create.as_ref().filter(|_| !at_rest),
        );
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
            match (self.drafts.profile_plane, self.drafts.profile_programs()) {
                (Some(plane), Ok(loops)) => {
                    self.ops.push(SessionOp::AddProfile { plane, loops });
                }
                // Unreachable while the button is gated on `blocked`,
                // and typed rather than unwrapped: a form's enabling
                // condition and its commit are two pieces of code, and
                // this one does not assume the other got it right.
                (None, _) => {
                    self.notices
                        .push(frame::tool_news("add profile: no frame picked"));
                }
                (_, Err(error)) => {
                    self.notices
                        .push(frame::tool_news(format!("add profile: {error}")));
                }
            }
        }
    }

    /// The extrude form: the current selection is the profile (a tree
    /// pick, or a face pick whose feature is one — `Selection::node`),
    /// one distance field, one [`SessionOp::AddExtrude`] on commit.
    /// A selection that is not a profile refuses typed at the door
    /// and lands on the status line.
    pub(crate) fn extrude_ui(&mut self, ui: &mut egui::Ui) {
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
                        Err(error) => {
                            self.notices
                                .push(frame::tool_news(format!("extrude: {error}")));
                        }
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
    pub(crate) fn revolve_tool_ui(&mut self, ui: &mut egui::Ui) {
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
    pub(crate) fn boolean_tool_ui(&mut self, ui: &mut egui::Ui) {
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
            // One button per operation the KERNEL has, in its order:
            // the form offers the vocabulary, never a copy of it.
            for &op in BooleanOp::ALL {
                ui.radio_value(&mut self.drafts.boolean_op, op, boolean_op_label(op));
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
    pub(crate) fn split_tool_ui(&mut self, ui: &mut egui::Ui) {
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
    pub(crate) fn transform_tool_ui(&mut self, ui: &mut egui::Ui) {
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
    /// fields, the axis pick the circular rule needs, the output
    /// choice, and the one committed edit.
    ///
    /// The output row is what fuses a pattern into the part: `fused`
    /// commits `Node::PlacedUnion`, whose ONE body every downstream
    /// seat consumes, where `instances` commits the several bodies a
    /// boolean seat refuses.
    pub(crate) fn pattern_tool_ui(&mut self, ui: &mut egui::Ui) {
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
            ui.label("output");
            for (output, label) in PatternOutputChoice::ALL {
                ui.radio_value(&mut self.drafts.pattern_output, output, label);
            }
        });
        ui.horizontal(|ui| {
            ui.label("count");
            // Clamped at one instance: a count is a structural slot
            // and a non-positive one refuses at evaluation, so the
            // form does not offer to author a node that cannot build.
            ui.add(
                number_field(&mut self.drafts.pattern_count, COUNT_DRAG_SPEED)
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
                    drafts.pattern_output,
                    drafts.pattern_count,
                    scalars(drafts.pattern_direction)?,
                    drafts.length(drafts.pattern_spacing)?,
                )?),
                PatternKindChoice::Circular => Ok(tool.circular_op(
                    drafts.pattern_output,
                    drafts.pattern_count,
                    drafts.angle(drafts.pattern_step)?,
                )?),
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
    pub(crate) fn blend_tool_ui(&mut self, ui: &mut egui::Ui) {
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
    pub(crate) fn all_edges_row(&mut self, ui: &mut egui::Ui, held: Option<BlendTarget>) {
        let target = held.or_else(|| BlendTarget::of_selection(self.session.selection()));
        let ready = target.zip(self.session.evaluation()).zip(self.index);
        let Some(((target, eval), index)) = ready else {
            ui.add_enabled(false, egui::Button::new("Select all edges"))
                .on_disabled_hover_text(
                    "click an edge or a face of the body first, and let it evaluate — \
                     a feature picked in the tree does not say which body",
                );
            return;
        };
        let clicked = ui
            .button("Select all edges")
            .on_hover_text(
                "every edge of this body as it stands now, stored as a frozen set — \
                 whether the kernel can BLEND that set is its own answer, on the node's badge",
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
            self.notices
                .push(frame::tool_news(ToolKind::Blend.says(&event)));
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
    pub(crate) fn blend_commit_row(&mut self, ui: &mut egui::Ui, count: usize) {
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
                                self.notices
                                    .push(frame::tool_news(ToolKind::Blend.says(&error)));
                            }
                            None => {}
                        }
                    }
                    Err(error) => {
                        self.notices
                            .push(frame::tool_news(ToolKind::Blend.says(&error)));
                    }
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
    pub(crate) fn tool_commit_row(
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
                    Err(error) => {
                        self.notices.push(frame::tool_news(kind.says(&error)));
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
    }
}
