//! The Properties pane: what the current selection is, and the slot,
//! parameter and instance fields that edit it.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{Axis3, Dimension, Frame, ParamName, RecipeNodeId};
use pncad::quantity::{self, UnitDef};

use crate::app::{ViewerBehavior, chrome, indeterminate_wording};
use crate::display::free_move_check;
use crate::forms::{FIELD_DRAG_SPEED, FieldWriting};
use crate::props::{self, ParamRow, SlotDriver, SlotGroup, SlotRow, SlotValue};
use crate::session::{BoundsTarget, Refusal, Selection, SessionOp, Standing};
use crate::widgets::{delete_button, drag_gesture_ops, drag_ops};

impl ViewerBehavior<'_> {
    /// The property panel.
    pub(crate) fn properties_ui(&mut self, ui: &mut egui::Ui) {
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
                    // Shown, scrubbed and authored in the unit the
                    // parameter was DECLARED in, through the same
                    // value a slot field is written by — a parameter
                    // written in millimetres reads in millimetres.
                    let field = FieldWriting::of(row.dimension, row.unit);
                    let mut value = field.shown(row.value.as_f64());
                    ui.horizontal(|ui| {
                        let widget = ui.add(egui::DragValue::new(&mut value).speed(field.tick));
                        // **A LABEL, where a slot row has a picker.**
                        // Not "the way a slot row says it": a slot's
                        // unit is said by a `ComboBox` that CHANGES it
                        // (`slot_unit_ui`), and `length_picker`'s rule
                        // — the unit is the picker's to say, not the
                        // field's — is why nothing else beside a slot
                        // field states it. A parameter has no such
                        // picker to be the one that says it, because
                        // there is no edit for one to push
                        // (`work/issues/doc-param-unit-edit-has-no-door.md`),
                        // and a number with no notation beside it at
                        // all is worse than a notation nobody can
                        // change. The dimensionless row and a `Count`
                        // have nothing to say here.
                        if let Some(unit) = field.unit.filter(|u| !u.symbol().is_empty()) {
                            ui.weak(unit.symbol());
                        }
                        drag_ops(
                            &widget,
                            field.authored(value),
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
                    });
                    self.param_bounds_ui(ui, &row);
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
    pub(crate) fn add_param_ui(&mut self, ui: &mut egui::Ui) {
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
            // The form authors in the canonical unit — a new
            // parameter's declaration names that notation
            // (`props::doc_param`) — so the tick is the canonical one
            // for the dimension picked, and the panel's own rule
            // answers it rather than a constant beside it. With no
            // dimension picked yet there is no tick to derive and
            // Create is refused anyway; a length's serves as the
            // placeholder.
            let speed = self
                .drafts
                .new_param_dimension
                .map_or(FIELD_DRAG_SPEED, |dimension| {
                    FieldWriting::of(dimension, None).tick
                });
            ui.add(egui::DragValue::new(&mut self.drafts.new_param_value).speed(speed));
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
    pub(crate) fn standing_ui(&mut self, ui: &mut egui::Ui, standing: &Standing) {
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
    pub(crate) fn entity_standing_ui(
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
                    indeterminate_wording(noun, cause),
                );
            }
        }
    }

    /// The selected instance's display controls: the hide toggle and
    /// the free-move probe. Draws nothing for a non-instance node —
    /// the section is about per-instance display state, which other
    /// nodes do not have.
    pub(crate) fn instance_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId) {
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
                // A LENGTH field written in millimetres — so the
                // conversion and the drag tick are the panel's own
                // ([`FieldWriting`]) rather than a factor of a thousand
                // and a bare `0.5` with nothing saying what unit they
                // are in. Three components of one frame, one writing.
                let field = FieldWriting::of(Dimension::Length, Some(quantity::MM.def()));
                let mut mm = current.map(|v| field.shown(v));
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
                        let widget = ui.add(egui::DragValue::new(&mut value).speed(field.tick));
                        mm[axis] = value;
                        let frame_of =
                            |mm: [f64; 3]| Frame::translation(mm.map(|v| field.authored(v)));
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
    pub(crate) fn slot_group_ui(
        &mut self,
        ui: &mut egui::Ui,
        node: RecipeNodeId,
        group: &SlotGroup,
    ) {
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
    /// The number is shown in the unit the slot is WRITTEN in, scrubbed
    /// at a tick in that same unit, and authored back through the same
    /// factor ([`FieldWriting`], the text door's one-multiply
    /// semantics), with NO unit suffix on the text: the picker beside
    /// the field names the unit, and saying it twice adjacently says
    /// it once.
    pub(crate) fn slot_value_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        // `Count` is the one row with no unit at all (an instance count
        // is a number, not a quantity), and its factor would be 1.0
        // anyway — so the absence is an identity here, not a fallback.
        let field = FieldWriting::of(row.dimension, row.unit);
        // A slot that did not evaluate still has SOURCE to edit — it
        // is the slot most likely to need it — so the field is drawn
        // for it too, over the one number it does not have. The fault
        // itself is said beside the field.
        if let Err(ref error) = row.value {
            ui.weak(format!("{error}"));
        }
        let mut number = field.shown(match row.value {
            Ok(value) => value.as_f64(),
            Err(_) => 0.0,
        });
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
            .speed(field.tick)
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
            field.authored(number),
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
                let value = SlotValue::of(row.dimension, field.authored(written));
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
    pub(crate) fn slot_unit_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, rows: &[SlotRow]) {
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
    pub(crate) fn slot_notes_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
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
        // Written in the unit the SEARCH used, which the reading
        // carries — not re-derived from the row beside it. One sentence
        // for a slot's range and a parameter's alike
        // (`BoundsReading::wording`); this row prefixes the slot's name
        // because a node draws several of them.
        if let Some(reading) = self.session.bounds()
            && reading.target == target
        {
            ui.weak(format!("{}: {}", row.slot.label(), reading.wording()));
        }
    }

    /// The button that asks for one slot's locally-valid range.
    ///
    /// **Asked for, not automatic** — see `SessionOp::ProbeBounds` for
    /// why. Offered only where a number can actually be written: a
    /// driven slot's value is not the user's to move, so a range for it
    /// would answer a question they cannot act on. The reading itself
    /// lands in [`Self::slot_notes_ui`], in the slot's own written unit.
    pub(crate) fn range_button(
        &mut self,
        ui: &mut egui::Ui,
        node: RecipeNodeId,
        row: &SlotRow,
        label: &str,
    ) {
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
    ///
    /// The reading is written in the unit the SEARCH ran in, which
    /// `BoundsReading` carries beside the range: the panel says the
    /// sentence, it does not decide the notation. That is what keeps
    /// "the range says millimetres because the search stepped
    /// millimetres" a fact about one value rather than an agreement
    /// between two reads.
    pub(crate) fn param_bounds_ui(&mut self, ui: &mut egui::Ui, row: &ParamRow) {
        let target = BoundsTarget::Param {
            name: row.name.clone(),
        };
        ui.horizontal(|ui| {
            if let Some(reading) = self.session.bounds()
                && reading.target == target
            {
                ui.weak(reading.wording());
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
}
