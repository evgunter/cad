//! The Properties pane: what the current selection is, and the slot,
//! parameter and instance fields that edit it.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{Axis3, Dimension, Frame, Node, ParamName, RecipeNodeId};
use pncad::quantity::{self, UnitDef};

use crate::app::{ViewerBehavior, chrome, indeterminate_wording};
use crate::display::free_move_check;
use crate::forms::{FIELD_DRAG_SPEED, FieldWriting};
use crate::frame::Tone;
use crate::props::{self, ParamRow, SlotDriver, SlotGroup, SlotRow, SlotValue};
use crate::session::{BoundsTarget, Refusal, Selection, SessionOp, Standing, ValueGestureName};
use crate::theme::Theme;
use crate::widgets::{
    FieldShowing, FieldVocabulary, ProbeOps, UNIT_PICKER_WIDTH, angle_picker, delete_button,
    free_move_gesture, length_picker, number_field, pick_unit, unit_field, value_field_ops,
    value_gesture, vec3_row_ops,
};

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
                    crate::widgets::message_toned(
                        ui,
                        "this feature carries no parameters",
                        &self.theme,
                        Tone::Advisory,
                    );
                }
                self.feature_rows_ui(ui, node, &groups);
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
                    crate::widgets::message_toned(
                        ui,
                        "this feature carries no parameters",
                        &self.theme,
                        Tone::Advisory,
                    );
                }
                // `Selection::node()` is the one inversion — always
                // `Some` on these two arms, and read rather than
                // re-derived so the rows and the edits land on the
                // node `slot_groups` answered for.
                if let Some(feature) = self.session.selection().node() {
                    self.feature_rows_ui(ui, feature, &groups);
                }
            }
            Selection::Param(name) => {
                if let Some(row) = crate::props::param_rows(self.session.doc())
                    .into_iter()
                    .find(|row| row.name == name)
                {
                    // The dimension in the common noun editor-core's
                    // `Display` spells, never the variant identifier:
                    // a label a person reads is prose.
                    crate::widgets::message(
                        ui,
                        format!("parameter {} ({})", row.name.0, row.dimension),
                    );
                    // Shown, scrubbed and authored in the unit the
                    // parameter was DECLARED in, through the same
                    // value a slot field is written by — a parameter
                    // written in millimetres reads in millimetres.
                    let field = FieldWriting::of(row.dimension, row.unit);
                    ui.horizontal(|ui| {
                        // **The two doors a parameter's field has.** A
                        // bare number is a value in the notation the
                        // field is written in and nothing else moves;
                        // anything else is text for
                        // `SessionOp::SetParamText`, which reads a
                        // number and its notation through the one
                        // parser and refuses what is neither. The
                        // panel parses nothing, and the field itself
                        // is the slot row's — one function, because
                        // the two rows differ only in which operation
                        // each door spells.
                        //
                        // **A parameter always shows its number.** It
                        // is never driven by anything, so there is no
                        // source for the field to show instead and no
                        // fixed text to pin over it.
                        value_field_ops(
                            ui,
                            FieldShowing {
                                writing: field,
                                dimension: row.dimension,
                                number: props::shown_value(field.unit, row.value.as_f64()),
                                text: None,
                            },
                            value_gesture(ValueGestureName::Param(name.clone())),
                            FieldVocabulary {
                                number: |value| SessionOp::SetParam {
                                    name: name.clone(),
                                    value,
                                },
                                text: |text| SessionOp::SetParamText {
                                    name: name.clone(),
                                    text,
                                },
                            },
                            self.ops,
                        );
                        // **The unit is the picker's to say.** A
                        // parameter's notation is a fact the document
                        // stores and an edit changes
                        // (`SessionOp::SetParamUnit`, over
                        // `DocEdit::SetDocParamUnit`), so the row says
                        // it the way a slot row does: with the control
                        // that changes it. The dimensionless row and a
                        // `Count` have no notation to offer and draw
                        // nothing.
                        self.param_unit_ui(ui, &row);
                    });
                    self.param_bounds_ui(ui, &row);
                } else {
                    crate::widgets::message_toned(
                        ui,
                        "that parameter is gone",
                        &self.theme,
                        Tone::Advisory,
                    );
                }
            }
        }
        ui.separator();
        ui.label("document parameters");
        for row in crate::props::param_rows(self.session.doc()) {
            // A name the user authored, so nothing bounds its width.
            if crate::widgets::message_link(ui, row.name.0.clone()).clicked() {
                self.ops
                    .push(SessionOp::Select(Selection::Param(row.name.clone())));
            }
        }
        self.add_param_ui(ui);
    }

    /// **A feature's editing rows**: its slot rows — and, for a
    /// profile, the add-profile form's own editor above them
    /// ([`ViewerBehavior::edit_profile_ui`]).
    ///
    /// Under the editor the slot rows are FOLDED, not dropped: they are
    /// the door for what the editor's number fields do not carry —
    /// driving an argument by an expression, re-noting its unit,
    /// probing its range. A profile the editor cannot hold (an argument
    /// already driven) shows its refusal and the rows open.
    fn feature_rows_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, groups: &[SlotGroup]) {
        let profile = matches!(
            self.session.committed_doc().node(node),
            Some(Node::Profile(_))
        );
        if profile && self.edit_profile_ui(ui, node) {
            egui::CollapsingHeader::new("arguments")
                .id_salt(("profile_arguments", node.0))
                .show(ui, |ui| {
                    crate::widgets::message_toned(
                        ui,
                        "each argument as a slot: drive it by an expression, change the unit it \
                         is written in, or probe its range — an edit here reloads the editor above",
                        &self.theme,
                        Tone::Advisory,
                    );
                    for group in groups {
                        self.slot_group_ui(ui, node, group);
                    }
                });
        } else {
            for group in groups {
                self.slot_group_ui(ui, node, group);
            }
        }
    }

    /// The create half of the document-parameters section: name,
    /// dimension, value, the NOTATION to write it in, one
    /// [`SessionOp::CreateParam`] on commit.
    ///
    /// **The unit is the form's, not this form's.** A length picked
    /// here is `Drafts::length_unit`, the one every creation form in
    /// the crate writes its lengths in — a form's notation is a
    /// statement about how the person at the keyboard is working, and
    /// somebody declaring a parameter in millimetres is not then
    /// authoring the extrude that consumes it in metres. The panel's
    /// pickers are per literal for the opposite reason, and a
    /// parameter's DECLARED notation is one of those: it is changed
    /// afterwards at its own row ([`ViewerBehavior::param_unit_ui`]),
    /// not here.
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
                crate::widgets::message_toned(
                    ui,
                    Refusal::offer_wording(&offered),
                    &self.theme,
                    Tone::Advisory,
                );
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
            // One button per dimension the KERNEL has, in its order
            // and under its own word for it: the form offers the
            // vocabulary and writes no copy of it — neither the
            // membership, which is `Dimension::ALL`, nor the words,
            // which are the `Display` that `editor_core::expr` calls
            // the one home of the dimension-in-prose rule. A fifth
            // dimension therefore arrives in this row with no edit
            // here, and it arrives as the noun a person would say
            // rather than as a capitalised variant identifier, which
            // is what these four buttons used to read as.
            for dimension in Dimension::ALL {
                ui.radio_value(
                    &mut self.drafts.new_param_dimension,
                    Some(dimension),
                    dimension.to_string(),
                );
            }
            // **The form authors in the notation the picker says**, and
            // the draft behind the field stays canonical whatever it
            // says (`widgets::unit_field`) — so the tick handed over is
            // the CANONICAL one for the dimension picked and the field
            // divides it by the same factor it divides the value by.
            // The tick a person feels is therefore
            // `FieldWriting::of(dimension, unit).tick`, derived rather
            // than stated, and applying the factor here as well would
            // apply it twice. With no dimension picked yet there is no
            // tick to derive and Create is refused anyway; a length's
            // serves as the placeholder.
            let speed = self
                .drafts
                .new_param_dimension
                .map_or(FIELD_DRAG_SPEED, |dimension| {
                    FieldWriting::of(dimension, None).tick
                });
            match self.new_param_unit() {
                Some(unit) => unit_field(ui, unit, speed, &mut self.drafts.new_param_value),
                // A `Count` and a bare `Scalar` name no notation, so
                // the field is the number itself and no picker is
                // drawn beside it.
                None => {
                    ui.add(number_field(&mut self.drafts.new_param_value, speed));
                }
            }
            // Drawn AFTER the field it governs, which is the forms'
            // rule (`widgets::length_picker`): the pick is an input
            // event, so the field it re-writes is next frame's.
            match self.drafts.new_param_dimension {
                Some(Dimension::Length) => {
                    length_picker(ui, "add_param", &mut self.drafts.length_unit);
                }
                Some(Dimension::Angle) => {
                    angle_picker(ui, "add_param", &mut self.drafts.angle_unit);
                }
                Some(Dimension::Scalar | Dimension::Count) | None => {}
            }
        });
        let name = self.drafts.new_param_name.trim();
        let existing = if name.is_empty() {
            None
        } else {
            self.session.doc().params().get(&ParamName::new(name))
        };
        if let Some(existing) = existing {
            let name = ParamName::new(name);
            if exists_notice(ui, &self.theme, &name, existing.dim()) {
                self.ops.push(SessionOp::Select(Selection::Param(name)));
            }
            return;
        }
        let ready = !name.is_empty() && self.drafts.new_param_dimension.is_some();
        let create = ui.add_enabled(ready, egui::Button::new("Create"));
        let create = if self.drafts.new_param_dimension.is_none() {
            create.on_disabled_hover_text("pick a dimension first")
        } else {
            create
        };
        // The draft value is asked whether the dimension can carry it
        // before a declaration is minted from it: a `Count` parameter
        // declared from a field holding `NaN` would otherwise be
        // created holding zero, which is a value nobody authored.
        // `SlotValue::of` is the one door that decides this.
        if create.clicked()
            && let Some(dimension) = self.drafts.new_param_dimension
            && let Ok(value) = SlotValue::of(dimension, self.drafts.new_param_value)
        {
            self.ops.push(SessionOp::CreateParam {
                name: ParamName::new(name),
                value: crate::props::doc_param(dimension, value, self.new_param_unit()),
            });
            self.drafts.new_param_name.clear();
            self.drafts.new_param_dimension = None;
            self.drafts.new_param_value = 0.0;
            self.drafts.new_param_offer = None;
        }
    }

    /// **The notation the add-parameter form is authoring in** — the
    /// form's own unit for the dimension picked, and `None` where
    /// there is no notation to name (a `Count`, a bare `Scalar`, or no
    /// dimension picked yet).
    ///
    /// One answer read by TWO places — the field beside it and the
    /// declaration the Create button mints — so the number on screen
    /// and the unit the document remembers cannot disagree.
    ///
    /// **The picker is a third place and does not read it**, which is
    /// the honest state of this form. `length_picker` and
    /// `angle_picker` write a typed draft (`Drafts::length_unit`,
    /// `angle_unit`), so the ladder below is spelled a second time to
    /// choose between them and the two could disagree. It is an
    /// instance of the crate's `Dimension`-to-unit ladder class, filed
    /// on CHROME, and the pairing it protects — a length picker that
    /// could write a `deg` — is the one the typed drafts already make
    /// unrepresentable.
    fn new_param_unit(&self) -> Option<UnitDef> {
        match self.drafts.new_param_dimension? {
            Dimension::Length => Some(self.drafts.length_unit.def()),
            Dimension::Angle => Some(self.drafts.angle_unit.def()),
            Dimension::Scalar | Dimension::Count => None,
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
                    ui.label(crate::tree::node_number(*node));
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
                    crate::widgets::message_toned(
                        ui,
                        format!("parameter {} is no longer declared", name.0),
                        &self.theme,
                        Tone::Actionable,
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
            ui.label(format!("{noun} of {}", crate::tree::node_number(feature)));
            if live && delete_button(ui, self.session, feature) {
                self.ops.push(SessionOp::DeleteNode { node: feature });
            }
        });
        // The typed verdict, rendered from the resolution machinery's
        // own payload — never a sentence composed here about somebody
        // else's refusal.
        match resolution {
            None => {
                crate::widgets::message_toned(
                    ui,
                    "no evaluation yet to resolve this against",
                    &self.theme,
                    Tone::Advisory,
                );
            }
            Some(pncad::select::Resolution::Resolved(_)) => {}
            Some(pncad::select::Resolution::Failed(failure)) => {
                crate::widgets::message_toned(
                    ui,
                    format!("this {noun} is gone: {}", failure.error),
                    &self.theme,
                    Tone::Actionable,
                );
                if !failure.offers.is_empty() {
                    // A count and a fixed literal, so a name.
                    ui.weak(format!(
                        "{} rebind candidate(s) offered",
                        failure.offers.len()
                    ));
                }
            }
            Some(pncad::select::Resolution::Indeterminate(cause)) => {
                crate::widgets::message_toned(
                    ui,
                    indeterminate_wording(noun, cause),
                    &self.theme,
                    Tone::Actionable,
                );
            }
        }
    }

    /// The selected instance's display controls: the hide toggle and
    /// the free-move probe. Draws nothing for a node the document does
    /// not admit display state on — the section is about per-instance
    /// display state, which neither another kind of node nor an id the
    /// document no longer holds has.
    ///
    /// **Silence is the whole answer for both refusals, and for the
    /// absent id it is half of a rule rather than a discard.** A
    /// selection outlives the thing it names ([`Standing`]: *a vanished
    /// reference is a STATE, not an event*), so this door really is
    /// reached with an id the document no longer holds — and in that
    /// frame [`Self::standing_ui`] has already drawn the vanished
    /// verdict, from the same `doc().node(..)` lookup, directly above
    /// this section. The rule's other clause is that *the affordances
    /// that need a live entity switch off*, which is this. Saying it
    /// again here would be one fact spelled twice in one pane, which is
    /// what the parameter half of this panel already does and is not a
    /// pattern to copy.
    pub(crate) fn instance_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId) {
        // The fault is discarded HERE, at the party that decides the
        // two refusals are one answer, rather than by a door that
        // answered a `bool` and could not have offered anything else.
        if crate::display::instance_check(self.session.doc(), node).is_err() {
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
                crate::widgets::message_toned(ui, fault.to_string(), &self.theme, Tone::Advisory);
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
                // **A conversion above the widget, and the one in this
                // crate that cannot fail.** A millimetre value leaves
                // `f64` above `f64::MAX * MILLI`
                // ([`crate::props::written`]), which is why the panel's
                // value fields ask before they convert. The translation
                // here is not a document value: it is one this probe
                // itself authored, either out of this field — bounded
                // by what a finite millimetre text can spell, which is
                // `1e305` m short of the overflow — or out of a pointer
                // drag in world coordinates. A door that minted a frame
                // from a document value would make this
                // `crate::props::shown_value` like the other two.
                let mut mm = current.map(|v| field.shown(v));
                // The G1 gesture triple over DISPLAY state, through the
                // one widget→gesture mapping (`drag_ops`) so the typed-
                // input arm exists here too: typing a value performs a
                // one-shot begin/preview/commit, exactly one committed
                // display value. The instance has ONE probe and all
                // three components drive it, so the row is one gesture
                // and `vec3_row_ops` maps it once — its docs carry what
                // a triple per box costs. Each preview composes the
                // FULL frame from all three, so dragging x does not
                // zero y and z. The chrome offers the translation
                // components; the op vocabulary takes any rigid frame.
                let frame_of = |mm: [f64; 3]| Frame::translation(mm.map(|v| field.authored(v)));
                let ProbeOps { gesture, typed } = free_move_gesture(node, frame_of);
                ui.horizontal(|ui| {
                    vec3_row_ops(ui, field.tick, &mut mm, gesture, typed, self.ops);
                });
            }
        }
    }

    /// One PANEL ROW: a scalar slot on its own, or a 3-vector's three
    /// components on one line.
    ///
    /// The grouping is [`crate::props::SlotGroup`]'s and the vocabulary's (see
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
                    ui.weak(row.dimension.to_string());
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
                    ui.weak(family.dimension().to_string());
                });
                // The notes stay PER COMPONENT: an affordance names the
                // parameters driving one component, and a range is one
                // field's. Each names its component's slot.
                for row in rows.iter() {
                    self.slot_notes_ui(ui, node, row);
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
    /// is read once, by [`crate::props::field_edit`], and takes one of two
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
        // itself is said UNDER the row ([`slot_notes`]): this field is
        // drawn in its group's row, and a sentence there would be laid
        // out from the field's right-hand edge.
        // **The conversion is where the refusal is asked.** A slot the
        // notation cannot name has no number for the field to show
        // ([`crate::props::shown_value`]), and a slot that did not
        // evaluate has no number at all — zero is what the widget
        // holds for it, under the source text the row shows instead.
        let number = props::shown_value(
            field.unit,
            match row.value {
                Ok(value) => value.as_f64(),
                Err(_) => 0.0,
            },
        );
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
        // **The panel's value field, both doors and the gesture** —
        // the parameter row's field is this same call with its own
        // two operations. What a slot contributes is the fixed text
        // above: a row showing SOURCE rather than a number echoes
        // that source, and a number typed over it is no echo of
        // anything, so the driven slot's refusal stays reachable and
        // is owed its affordance even when the number happens to
        // match.
        value_field_ops(
            ui,
            FieldShowing {
                writing: field,
                dimension: row.dimension,
                number,
                text: fixed,
            },
            value_gesture(ValueGestureName::Slot {
                node,
                slot: row.slot,
            }),
            FieldVocabulary {
                number: |value| SessionOp::SetSlot {
                    node,
                    slot: row.slot,
                    value,
                },
                text: |text| SessionOp::SetSlotExpression {
                    node,
                    slot: row.slot,
                    text,
                },
            },
            self.ops,
        );
    }

    /// The written-unit picker for a document parameter's row.
    ///
    /// **"How do I want this number written" is an edit**, here as at
    /// a slot row: the notation rides on the declaration and persists,
    /// so the picker emits `SessionOp::SetParamUnit` and the change
    /// enters the history like any other.
    ///
    /// Nothing is drawn for a dimension with no units (`Scalar`,
    /// `Count`) — there is no notation to offer for a number that is
    /// not a quantity. Both halves of that answer come from
    /// `props`: `rendering_unit` says what the row is written in and
    /// answers `None` for exactly those dimensions, and
    /// `widgets::pick_unit` reads their options off
    /// `props::unit_options`, which is the same closed table.
    ///
    /// The combo itself is `widgets::pick_unit`, the creation forms'
    /// — the options, the width and the selected row are one
    /// question at every picker in the chrome. What this row adds is
    /// that a pick is an EDIT, and that re-picking the row already
    /// shown is not one.
    pub(crate) fn param_unit_ui(&mut self, ui: &mut egui::Ui, row: &ParamRow) {
        let Some(written) = props::rendering_unit(row.dimension, row.unit) else {
            return;
        };
        if let Some(unit) = pick_unit(ui, "param_unit", &row.name.0, row.dimension, written)
            && unit != written
        {
            self.ops.push(SessionOp::SetParamUnit {
                name: row.name.clone(),
                unit,
            });
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
            .selected_text(label)
            // The pickers' one width (`widgets::UNIT_PICKER_WIDTH`):
            // this combo draws the same table's symbols and cannot be
            // narrower than they are.
            .width(UNIT_PICKER_WIDTH)
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

    /// What a slot has to SAY, under its row ([`slot_notes`]), with the
    /// session's range reading when one has been taken for this field.
    pub(crate) fn slot_notes_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        let target = BoundsTarget::Slot {
            node,
            slot: row.slot,
        };
        let reading = self.bounds_wording(&target);
        if let Some(name) = slot_notes(ui, &self.theme, row, reading.as_deref()) {
            self.ops.push(SessionOp::Select(Selection::Param(name)));
        }
    }

    /// **The session's range reading for `target`, as its sentence** —
    /// `None` while no reading has been taken for that field.
    ///
    /// Written in the unit the SEARCH used, which the reading carries,
    /// not re-derived from the row it is drawn under: one sentence for
    /// a slot's range and a parameter's alike
    /// (`BoundsReading::wording`).
    fn bounds_wording(&self, target: &BoundsTarget) -> Option<String> {
        self.session
            .bounds()
            .filter(|reading| reading.target == *target)
            .map(|reading| reading.wording())
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
            button.on_hover_text(PROBE_HOVER)
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
        let reading = self.bounds_wording(&target);
        if bounds_notes(ui, &self.theme, reading.as_deref()) {
            self.ops.push(SessionOp::ProbeBounds { target });
        }
    }
}

/// **The already-declared notice and the edit door it offers**, each
/// on a line of its own.
///
/// Both are sentences: the wording quotes a name the user typed, and
/// so does the door. Neither shares a row with anything, because a
/// sentence beside a control is laid out from the control's right-hand
/// edge and a sentence beside a sentence is drawn past the pane's.
///
/// Answers whether the door was clicked.
fn exists_notice(ui: &mut egui::Ui, theme: &Theme, name: &ParamName, dimension: Dimension) -> bool {
    // The same sentence the session's refusal would show, and the edit
    // door it offers instead — refuse-then-offer, ahead of the click.
    crate::widgets::message_toned(
        ui,
        Refusal::exists_wording(name, dimension),
        theme,
        Tone::Advisory,
    );
    crate::widgets::message_link(ui, format!("edit {}", name.0)).clicked()
}

/// **What a slot has to SAY, under its row**: the fault a slot that
/// did not evaluate carries, the expression-driven refusal's
/// affordance and its edit doors, and the range `reading` when one has
/// been taken for this field.
///
/// Every one of them is drawn under the row rather than in it. The
/// row holds the slot's fields — three, for a vector — and a sentence
/// in it would be laid out from the last field's right-hand edge. Each
/// note names the slot it is about, because a vector says them once per
/// component.
///
/// The affordance is attached to the row rather than raised on
/// refusal alone so the user can see WHY the number will not move
/// before they fight it — the refusal itself still surfaces in the
/// status line when they try. Its wording is the ratified one, from
/// its one home, the same string the status line shows when the edit
/// is actually attempted. Its doors share a WRAPPING row, one per
/// parameter the expression reads, so a door too long for what is
/// left of the line starts the next one.
///
/// Answers the parameter whose door was clicked.
fn slot_notes(
    ui: &mut egui::Ui,
    theme: &Theme,
    row: &SlotRow,
    reading: Option<&str>,
) -> Option<ParamName> {
    if let Err(error) = &row.value {
        crate::widgets::message_toned(
            ui,
            format!("{}: {error}", row.slot.label()),
            theme,
            Tone::Advisory,
        );
    }
    let mut clicked = None;
    if let SlotDriver::Expression { params } = &row.driver {
        crate::widgets::message_toned(
            ui,
            format!(
                "{}: {}",
                row.slot.label(),
                Refusal::affordance(params, row.value.as_ref().ok().copied())
            ),
            theme,
            Tone::Advisory,
        );
        if !params.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for name in params {
                    if crate::widgets::message_link(ui, format!("edit {}", name.0)).clicked() {
                        clicked = Some(name.clone());
                    }
                }
            });
        }
    }
    if let Some(reading) = reading {
        crate::widgets::message_toned(
            ui,
            format!("{}: {reading}", row.slot.label()),
            theme,
            Tone::Advisory,
        );
    }
    clicked
}

/// **A document parameter's range `reading` and the button that takes
/// one** — the reading on its own line, the button under it, which is
/// the order a slot's reading ([`slot_notes`]) and its range row are
/// drawn in.
///
/// The reading is a sentence whose numbers are the search's, so it is
/// not drawn beside the button: there it would be laid out from the
/// button's edge rather than the pane's.
///
/// Answers whether the button was clicked.
fn bounds_notes(ui: &mut egui::Ui, theme: &Theme, reading: Option<&str>) -> bool {
    if let Some(reading) = reading {
        crate::widgets::message_toned(ui, reading, theme, Tone::Advisory);
    }
    ui.small_button("range?")
        .on_hover_text(PROBE_HOVER)
        .clicked()
}

/// What a range button says on hover, for a slot's and a parameter's
/// alike.
const PROBE_HOVER: &str =
    "probe how far this can move before something new fails (tens of evaluations)";

/// **Where this pane's sentences LAND** — each measured in a pane
/// narrower than the sentence, through `crate::pane::headless::landed`,
/// as `crate::widgets::message_tests` measures the widget itself.
///
/// What each row holds is the layout, not only the widget: a sentence
/// on a line of its own, every row of it inside the pane and starting
/// at the pane's left edge, and the control it is about on a line
/// above or below it rather than beside it. A sentence drawn beside a
/// control in a `ui.horizontal` is laid out from the control's
/// right-hand edge at infinite width, and fails the first of those.
#[cfg(test)]
mod layout_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::document::{Dimension, ParamName, SlotId};

    use super::{bounds_notes, exists_notice, slot_notes};
    use crate::pane::headless::{assert_inside, assert_own_lines, assert_under, drawn_in, find};
    use crate::props::{SlotDriver, SlotFault, SlotRow, SlotValue};
    use crate::session::Refusal;
    use crate::theme::Theme;

    /// A narrow pane, and wider than `crate::widgets::message_floor`,
    /// so these rows read the region and not the floor.
    const REGION: f32 = 260.0;

    fn param(name: &str) -> ParamName {
        ParamName(name.to_owned())
    }

    /// An extrude distance row with `driver` and `value`.
    fn distance_row(driver: SlotDriver, value: Result<SlotValue, SlotFault>) -> SlotRow {
        SlotRow {
            slot: SlotId::Distance,
            dimension: Dimension::Length,
            structural: false,
            driver,
            value,
            unit: None,
            source: None,
        }
    }

    #[test]
    fn a_declared_names_notice_and_its_door_each_take_a_line_inside_the_pane() {
        let name = param("outer_enclosure_wall_thickness");
        let wording = Refusal::exists_wording(&name, Dimension::Length);
        let door = format!("edit {}", name.0);
        let (region, painted) = drawn_in(REGION, |ui| {
            exists_notice(ui, &Theme::DEFAULT, &name, Dimension::Length);
        });
        let notice = find(&painted, &wording);
        assert!(
            notice.rows.len() > 1,
            "the notice is longer than the pane, so it wraps ({:?})",
            notice.rows
        );
        assert_own_lines(region, notice);
        let edit = find(&painted, &door);
        assert_own_lines(region, edit);
        assert_under(notice, edit);
    }

    #[test]
    fn a_driven_slots_affordance_and_its_doors_stay_inside_the_pane() {
        let params = vec![
            param("outer_enclosure_wall_thickness"),
            param("lid_clearance"),
            param("gasket_compression_allowance"),
        ];
        let value = SlotValue::of(Dimension::Length, 0.004).expect("a finite length");
        let row = distance_row(
            SlotDriver::Expression {
                params: params.clone(),
            },
            Ok(value),
        );
        let (region, painted) = drawn_in(REGION, |ui| {
            slot_notes(ui, &Theme::DEFAULT, &row, None);
        });
        let affordance = find(
            &painted,
            &format!(
                "{}: {}",
                SlotId::Distance.label(),
                Refusal::affordance(&params, Some(value))
            ),
        );
        assert_own_lines(region, affordance);
        for name in &params {
            let door = find(&painted, &format!("edit {}", name.0));
            assert_inside(region, door);
            assert_under(affordance, door);
        }
    }

    #[test]
    fn a_slots_fault_is_said_under_its_row_inside_the_pane() {
        let row = distance_row(SlotDriver::Literal, Err(SlotFault::NoExpression));
        let (region, painted) = drawn_in(REGION, |ui| {
            slot_notes(ui, &Theme::DEFAULT, &row, None);
        });
        let fault = find(
            &painted,
            &format!("{}: {}", SlotId::Distance.label(), SlotFault::NoExpression),
        );
        assert_own_lines(region, fault);
    }

    #[test]
    fn a_parameters_range_reading_is_said_over_its_button_inside_the_pane() {
        let reading = "free from 0.0012345678901234567 m to 12.345678901234567 m \
                       before something new fails";
        let (region, painted) = drawn_in(REGION, |ui| {
            bounds_notes(ui, &Theme::DEFAULT, Some(reading));
        });
        let said = find(&painted, reading);
        assert_own_lines(region, said);
        assert_under(said, find(&painted, "range?"));
    }
}
