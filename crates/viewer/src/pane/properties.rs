//! The Properties pane: what the current selection is, and the slot,
//! parameter and instance fields that edit it.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{Axis3, Dimension, Frame, ParamName, RecipeNodeId, SlotId};
use pncad::quantity::UnitDef;
use pncad::select::Resolution;

use crate::app::{ViewerBehavior, indeterminate_wording, toned};
use crate::display::free_move_check;
use crate::forms::{FIELD_DRAG_SPEED, FieldWriting};
use crate::frame::Tone;
use crate::props::{self, Notation, ParamRow, SlotDriver, SlotGroup, SlotRow, SlotValue};
use crate::session::{
    BoundsTarget, NodeKindWanted, Refusal, Selection, SessionOp, Standing, ValueGestureName, admits,
};
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
                // `live()` for the reason the pick arm below reads it:
                // a deleted node has no parameters because it has no
                // node, and the header above has already said which.
                if groups.is_empty() && standing.live() {
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
                // **The previewed document here, deliberately**: this
                // row's field IS the drag, so it must show the value
                // the gesture is previewing. What decides whether the
                // row is drawn at all — the declaration — is the same
                // in both documents, because the only edit a preview
                // applies to a parameter is a value write
                // (`props::param_edit`), which cannot declare or
                // undeclare one.
                if let Some(row) = crate::props::param_rows(self.session.doc())
                    .into_iter()
                    .find(|row| row.name == name)
                {
                    // The dimension in the common noun editor-core's
                    // `Display` spells, never the variant identifier:
                    // a label a person reads is prose.
                    crate::widgets::message(
                        ui,
                        format!("parameter {} ({})", row.name.as_str(), row.dimension),
                    );
                    ui.horizontal(|ui| {
                        value_field_ops(
                            ui,
                            param_showing(&row, *self.notation),
                            value_gesture(ValueGestureName::Param(name.clone())),
                            param_doors(&name),
                            self.ops,
                            self.notices,
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
                }
                // No row is an undeclared parameter, and nothing is
                // drawn for it here: the header above has said so
                // ([`standing_verdict`]), loud, and this panel says a
                // fact once — the rule `failure_lines` keeps for a
                // failed tree row and `instance_ui` keeps for a
                // vanished instance. The header's `present` reads the
                // same document this lookup does
                // (`DocSession::standing`), so the two are one answer.
            }
        }
        ui.separator();
        ui.label("document parameters");
        for row in crate::props::param_rows(self.session.doc()) {
            // A name the user authored, so nothing bounds its width.
            if crate::widgets::message_link(ui, row.name.as_str().to_owned()).clicked() {
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
        let profile = admits(
            self.session.committed_doc().node(node),
            NodeKindWanted::Profile,
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
    /// **The unit is the working notation's, not this form's.** A
    /// length picked here is [`Notation::length`], the one every
    /// creation form in the crate writes its lengths in and every value
    /// nobody wrote reads in ([`props::Notation`]). The panel's
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
            if offered.as_str() == self.drafts.new_param_name.trim() {
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
                    FieldWriting::of(dimension, None, Notation::CANONICAL).tick
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
                    length_picker(ui, "add_param", &mut self.notation.length);
                }
                Some(Dimension::Angle) => {
                    angle_picker(ui, "add_param", &mut self.notation.angle);
                }
                Some(Dimension::Scalar | Dimension::Count) | None => {}
            }
        });
        // The draft text is offered to the one door that decides what
        // a parameter name is; a refused text leaves the control
        // disabled, and the sentence the refusal carries is not yet
        // shown beside it.
        let name = ParamName::new(self.drafts.new_param_name.trim()).ok();
        // `create_param` asks `committed_doc()`, so the notice ahead of
        // the click asks it too: a notice drawn from the previewed
        // document would be answering about a document the door will
        // not see.
        let existing = name
            .as_ref()
            .and_then(|name| self.session.committed_doc().params().get(name));
        if let (Some(name), Some(existing)) = (&name, existing) {
            if exists_notice(ui, &self.theme, name, existing.dim()) {
                self.ops
                    .push(SessionOp::Select(Selection::Param(name.clone())));
            }
            return;
        }
        let ready = name.is_some() && self.drafts.new_param_dimension.is_some();
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
            && let Some(name) = name
            && let Some(dimension) = self.drafts.new_param_dimension
            && let Ok(value) = SlotValue::of(dimension, self.drafts.new_param_value)
        {
            self.ops.push(SessionOp::CreateParam {
                name,
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
    /// `angle_picker` write the typed working notation
    /// ([`Notation::length`], [`Notation::angle`]), so the ladder below is spelled a second time to
    /// choose between them and the two could disagree. It is an
    /// instance of the crate's `Dimension`-to-unit ladder class, filed
    /// on CHROME, and the pairing it protects — a length picker that
    /// could write a `deg` — is the one the typed notation already makes
    /// unrepresentable.
    fn new_param_unit(&self) -> Option<UnitDef> {
        match self.drafts.new_param_dimension? {
            Dimension::Length => Some(self.notation.length.def()),
            Dimension::Angle => Some(self.notation.angle.def()),
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
            Standing::Empty | Standing::Param { .. } => {}
            Standing::Node { node, present } => {
                ui.horizontal(|ui| {
                    ui.label(crate::tree::node_number(*node));
                    if *present && delete_button(ui, self.session, *node) {
                        self.ops.push(SessionOp::DeleteNode { node: *node });
                    }
                    // Beside the number, which is the node it is about.
                    standing_verdict(ui, &self.theme, standing);
                });
                return;
            }
            Standing::Face { face, .. } => {
                self.entity_header_ui(ui, "face", face.feature(), standing);
            }
            Standing::Edge { edge, .. } => {
                self.entity_header_ui(ui, "edge", edge.feature(), standing);
            }
        }
        standing_verdict(ui, &self.theme, standing);
    }

    /// A picked entity's header line: which feature it belongs to, and
    /// the delete that feature offers. Its verdict is
    /// [`standing_verdict`]'s, on the lines under it.
    ///
    /// **One rendering for every kind of picked entity**, taking the
    /// noun as an argument: a face and an edge differ in what they are
    /// called and in nothing else this line does.
    fn entity_header_ui(
        &mut self,
        ui: &mut egui::Ui,
        noun: &str,
        feature: RecipeNodeId,
        standing: &Standing,
    ) {
        ui.horizontal(|ui| {
            // The feature that MADE the entity, so the button deletes
            // what the label names.
            ui.label(format!("{noun} of {}", crate::tree::node_number(feature)));
            if standing.live() && delete_button(ui, self.session, feature) {
                self.ops.push(SessionOp::DeleteNode { node: feature });
            }
        });
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
    /// again here would be one fact spelled twice in one pane, which
    /// this panel does nowhere.
    ///
    /// **The section's gate and the toggle's are two different tests,
    /// and each control reads the one its own door runs.**
    /// [`crate::display::instance_check`] decides whether there is a
    /// section at all — it is the kind test, and a node of another kind
    /// has no per-instance display state to show. What the hide toggle
    /// pushes runs [`crate::display::display_check`], the full
    /// admission test, so an instance whose geometry is fused into a
    /// drawn root with another's is a live instance with a section and
    /// no display operation that can address it: the toggle is gated on
    /// the door's test and carries the door's own sentence.
    pub(crate) fn instance_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId) {
        // **The COMMITTED document, because that is the one the doors
        // read.** `set_hidden` and `begin_free_move` are handed
        // `history.doc()`, and both ops are permitted during a value
        // gesture, so a panel reading the previewed document would be
        // answering a different question from the click it is standing
        // in for. Reading the same document makes the agreement
        // structural instead of circumstantial.
        let doc = self.session.committed_doc();
        // The fault is discarded HERE, at the party that decides the
        // two refusals are one answer, rather than by a door that
        // answered a `bool` and could not have offered anything else.
        if crate::display::instance_check(doc, node).is_err() {
            return;
        }
        ui.separator();
        ui.label(format!("instance {}", node.0));
        // The admission test `SetInstanceHidden` itself runs, read once
        // for the section: the toggle below is offered exactly where
        // the op would accept it, and the free-move probe runs this
        // same test before its own.
        let addressable = crate::display::display_check(doc, node);
        let mut shown = !self.display.hidden.contains(&node);
        if hide_toggle(ui, &self.theme, &addressable, &mut shown) {
            self.ops.push(SessionOp::SetInstanceHidden {
                instance: node,
                hidden: !shown,
            });
        }
        // The probe below would refuse this very fault —
        // `free_move_check` runs `display_check` first — so the section
        // ends at the toggle rather than saying it a second time.
        if addressable.is_err() {
            return;
        }
        match free_move_check(doc, node) {
            Err(fault) => {
                // The typed ineligibility, shown where the control
                // would be — the same sentence the op would refuse
                // with. Advisory for every fault that reaches here: the
                // admission faults end the section above, which leaves
                // a mate placing the instance — the document working
                // as written, which asks nothing of the reader.
                crate::widgets::message_toned(ui, fault.to_string(), &self.theme, Tone::Advisory);
            }
            Ok(()) => {
                let current = self
                    .display
                    .moved
                    .get(&node)
                    .map_or([0.0; 3], |frame| frame.translation);
                // A LENGTH field written in the working notation — so
                // the conversion and the drag tick are the panel's own
                // ([`FieldWriting`]) and the probe reads in the unit
                // every other value nobody wrote reads in. Three
                // components of one frame, one writing.
                let field = FieldWriting::of(
                    Dimension::Length,
                    Some(self.notation.length.def()),
                    *self.notation,
                );
                let unit = self.notation.length.def();
                ui.label(format!(
                    "free-move probe ({}, display only):",
                    unit.symbol()
                ));
                // **Asked, not formed**, as the panel's value fields
                // ask ([`crate::props::shown_value`]): a translation
                // dragged or typed in a coarse notation can have no
                // value in a finer one the notation was switched to.
                let mut written = [0.0; 3];
                for (shown, canonical) in written.iter_mut().zip(current) {
                    match crate::props::shown_value(field.unit, canonical) {
                        Ok(value) => *shown = value,
                        Err(unit) => {
                            crate::widgets::message(ui, crate::props::no_reading(unit));
                            return;
                        }
                    }
                }
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
                let frame_of =
                    |written: [f64; 3]| Frame::translation(written.map(|v| field.authored(v)));
                let ProbeOps { gesture, typed } = free_move_gesture(node, frame_of);
                ui.horizontal(|ui| {
                    vec3_row_ops(ui, field.tick, &mut written, gesture, typed, self.ops);
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
                    // ONE picker for the vector: the components of a
                    // point that are written at all are written in one
                    // unit, or the user is being told something they
                    // did not mean to say. The picker reports a
                    // disagreement rather than hiding it, and skips a
                    // computed component out loud (`slot_unit_ui`).
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
        let draft = (self.drafts.expr_target == Some((node, row.slot)))
            .then_some(self.drafts.expr_text.as_str());
        // **The panel's value field, both doors and the gesture** —
        // the parameter row's field is this same call with its own
        // two operations. What a slot contributes is what the field
        // shows and what its edit opens on ([`slot_showing`]): a
        // number typed over a driven slot's reading is no echo of
        // anything, so the driven slot's refusal stays reachable and
        // is owed its affordance even when the number happens to
        // match.
        value_field_ops(
            ui,
            slot_showing(row, draft, *self.notation),
            value_gesture(ValueGestureName::Slot {
                node,
                slot: row.slot,
            }),
            slot_doors(node, row.slot),
            self.ops,
            self.notices,
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
        let Some(written) = props::rendering_unit(row.dimension, row.unit, *self.notation) else {
            return;
        };
        if let Some(unit) = pick_unit(ui, "param_unit", row.name.as_str(), row.dimension, written)
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
    /// picker emits `SessionOp::SetSlotUnit`, one per component it
    /// writes.
    ///
    /// **Each component is gated on the op's own slot admission**
    /// ([`crate::session::DocSession::slot_unit_refusal`]), which is
    /// what `SetSlotUnit` refuses a slot with whatever unit is picked. A
    /// component driven by an expression has no written notation, and
    /// the op refuses it with `SlotUnitFault::NotALiteral`. The picker
    /// reads that refusal, not the row's driver, and carries its words
    /// rather than composing its own. Two answers of the op are not read
    /// here. A held value gesture refuses every `SetSlotUnit` in
    /// `DocSession::perform` before this admission runs, and a pick made
    /// then is refused loudly there. A unit that does not measure the
    /// slot is the op's to refuse at the pick, and every option offered
    /// comes off the slot's own dimension's table.
    ///
    /// **Over a vector, the picker writes the components that HAVE a
    /// notation and says which it skips.** A computed component shows
    /// its expression, not a number in some unit, so "the components
    /// of a point are written in one unit" is a statement about the
    /// literal ones; one driven component does not make that notation
    /// any less the user's to choose. So:
    ///
    /// - every component refused: the picker reads `computed` and is
    ///   drawn disabled, with the refusals' own sentences on its
    ///   disabled hover;
    /// - some refused: the picker is live over the rest, a pick writes
    ///   the rest, and its hover says which components a pick writes
    ///   and which it skips, followed by the skipped components'
    ///   refusals, before the pick is made;
    /// - none refused: the picker writes all of them.
    ///
    /// The selected text is the unit the WRITABLE components agree on,
    /// else `mixed` — a disagreement is reported rather than quietly
    /// normalized, and the document says what it says until someone
    /// picks. A driven component's canonical rendering is not a
    /// notation anyone chose, so it plays no part in that agreement.
    /// A picker with no writable component therefore shows no unit.
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
        // Each component with the unit it is shown in, or with what
        // `SetSlotUnit` would refuse it with.
        let mut writable: Vec<(&SlotRow, Option<UnitDef>)> = Vec::new();
        let mut refused: Vec<(&SlotRow, Refusal)> = Vec::new();
        for row in rows {
            match self.session.slot_unit_refusal(node, row.slot) {
                Some(refusal) => refused.push((row, refusal)),
                None => writable.push((
                    row,
                    props::rendering_unit(row.dimension, row.unit, *self.notation),
                )),
            }
        }
        let first_unit = writable.first().and_then(|(_, unit)| *unit);
        let common = writable
            .iter()
            .all(|(_, unit)| *unit == first_unit)
            .then_some(first_unit)
            .flatten();
        let label = match (common, writable.is_empty()) {
            (_, true) => "computed",
            (Some(unit), false) => unit.symbol(),
            (None, false) => "mixed",
        };
        // The refusals render themselves; what is composed here is
        // only which components a pick writes and which it skips.
        let refusals: Vec<String> = refused
            .iter()
            .map(|(_, refusal)| refusal.to_string())
            .collect();
        let refusals = refusals.join("\n");
        let written: Vec<String> = writable.iter().map(|(row, _)| row.slot.label()).collect();
        let skipped: Vec<String> = refused.iter().map(|(row, _)| row.slot.label()).collect();
        let skips = format!(
            "a pick writes {} and skips {}:\n{refusals}",
            written.join(", "),
            skipped.join(", "),
        );
        let picker = ui.add_enabled_ui(!writable.is_empty(), |ui| {
            // `id_salt` off the first component's slot: two vectors on
            // one node (a plane's origin and its normal) draw two
            // pickers, and egui identifies a popup by its id.
            egui::ComboBox::from_id_salt((node.0, format!("{:?}", first.slot), "unit"))
                .selected_text(label)
                // The pickers' one width (`widgets::UNIT_PICKER_WIDTH`):
                // this combo draws the same table's symbols and cannot
                // be narrower than they are.
                .width(UNIT_PICKER_WIDTH)
                .show_ui(ui, |ui| {
                    let mut picked = None;
                    for option in options {
                        let selected = common == Some(option);
                        if ui.selectable_label(selected, option.symbol()).clicked() && !selected {
                            picked = Some(option);
                        }
                    }
                    picked
                })
        });
        let combo = picker.inner;
        if !refused.is_empty() {
            // Disabled, the hover is why; live, it is what a pick does
            // and skips. egui reads exactly one of the two hooks.
            let _ = combo
                .response
                .clone()
                .on_disabled_hover_text(&refusals)
                .on_hover_text(&skips);
        }
        if let Some(unit) = combo.inner.flatten() {
            for (row, _) in &writable {
                self.ops.push(SessionOp::SetSlotUnit {
                    node,
                    slot: row.slot,
                    unit,
                });
            }
        }
    }

    /// What a slot has to SAY, under its row ([`slot_notes`]), with the
    /// session's range reading when one has been taken for this field.
    pub(crate) fn slot_notes_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        let target = BoundsTarget::Slot {
            node,
            slot: row.slot,
        };
        let reading = self.bounds_wording(&target);
        if let Some(name) = slot_notes(ui, &self.theme, row, reading.as_deref(), *self.notation) {
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
    ///
    /// **The control reads the refused operation's own value.**
    /// `Session::probe_bounds` refuses a driven slot through
    /// `guard_driven` with [`Refusal::DrivenByExpression`], and
    /// [`Self::probe_refusal`] answers that same value ahead of the
    /// click; the button gates on whether there is one and renders it
    /// through its own `Display`. **The row's VALUE is not a second
    /// conjunct**: a slot the document holds a bare literal for always
    /// evaluates, because `Node::slot_dimension_fault` — the one
    /// predicate the edit doors and the load walk both ask — refuses
    /// an expression whose dimension disagrees with the slot's. A gate
    /// that also read the value would owe a sentence for a state no
    /// document can be in.
    pub(crate) fn range_button(
        &mut self,
        ui: &mut egui::Ui,
        node: RecipeNodeId,
        row: &SlotRow,
        label: &str,
    ) {
        let refused = Self::probe_refusal(node, row, *self.notation);
        let button = ui.add_enabled(refused.is_none(), egui::Button::new(label).small());
        let button = match refused {
            None => button.on_hover_text(PROBE_HOVER),
            // The refusal renders itself; nothing here composes words.
            Some(refusal) => button.on_disabled_hover_text(refusal.to_string()),
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

    /// **What `SessionOp::ProbeBounds` would answer for this row, or
    /// `None` where it would accept** — a [`Refusal`], not a sentence.
    ///
    /// `guard_driven` builds this very value from this very row
    /// (`props::slot_rows`, the driver and the value it evaluated), so
    /// the disabled control's words are the refused operation's own by
    /// construction rather than by two compositions agreeing. A caller
    /// that wants the words asks the value for them.
    pub(crate) fn probe_refusal(
        node: RecipeNodeId,
        row: &SlotRow,
        notation: Notation,
    ) -> Option<Refusal> {
        match &row.driver {
            SlotDriver::Literal => None,
            SlotDriver::Expression { params } => Some(Refusal::DrivenByExpression {
                node,
                slot: row.slot,
                params: params.clone(),
                current: row.value.as_ref().ok().copied(),
                notation,
            }),
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

/// **What the selection's standing has to SAY, drawn**: the verdict on
/// a selection that no longer denotes — a deleted node, an undeclared
/// parameter, a picked entity whose name did not resolve — in the
/// voice [`Standing::tone`] gives it, and the rebind count under a
/// failed name. Draws nothing for a selection that still denotes, or
/// for none.
///
/// A picked entity's noun is read off its own arm, so no caller can
/// hand this "face" for an edge.
///
/// The words are composed per arm — for a picked entity, the
/// resolution machinery's own payload, never a sentence composed here
/// about somebody else's refusal. How LOUD they are is not composed
/// per arm: it is read once, off the value.
///
/// A free function over the `Ui` so a headless drive can reach it
/// (`crate::pane::headless`).
pub(crate) fn standing_verdict(ui: &mut egui::Ui, theme: &Theme, standing: &Standing) {
    let tone = standing.tone();
    let (noun, resolution) = match standing {
        Standing::Empty
        | Standing::Node { present: true, .. }
        | Standing::Param { present: true, .. } => return,
        // One word, beside the node number the caller drew.
        Standing::Node { present: false, .. } => {
            ui.label(toned("deleted", theme, tone));
            return;
        }
        Standing::Param {
            name,
            present: false,
        } => {
            crate::widgets::message_toned(
                ui,
                format!("parameter {} is no longer declared", name.as_str()),
                theme,
                tone,
            );
            return;
        }
        Standing::Face { resolution, .. } => ("face", resolution.as_deref()),
        Standing::Edge { resolution, .. } => ("edge", resolution.as_deref()),
    };
    let said = match resolution {
        None => Some("no evaluation yet to resolve this against".to_owned()),
        Some(Resolution::Resolved(_)) => None,
        Some(Resolution::Failed(failure)) => {
            Some(format!("this {noun} is gone: {}", failure.error))
        }
        Some(Resolution::Indeterminate(cause)) => Some(indeterminate_wording(noun, cause)),
    };
    if let Some(said) = said {
        crate::widgets::message_toned(ui, said, theme, tone);
    }
    if let Some(Resolution::Failed(failure)) = resolution
        && !failure.offers.is_empty()
    {
        // A count and a fixed literal, so a name. Weak as secondary
        // text rather than as a tone: the verdict above it carries the
        // tone, and this line only counts what that verdict offers.
        ui.weak(format!(
            "{} rebind candidate(s) offered",
            failure.offers.len()
        ));
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
    crate::widgets::message_link(ui, format!("edit {}", name.as_str())).clicked()
}

/// **What a slot's value field shows, and what its keyboard edit
/// opens on** — `draft` is the refused text the field holds for a
/// re-type, when it holds one, and it is shown as typed.
///
/// The number is in the unit the slot is WRITTEN in ([`FieldWriting`]).
/// A slot that did not evaluate is still drawn, since its source is
/// what there is to fix, over a zero it does not show. A literal that
/// evaluated shows no fixed text: egui formats the number it is
/// dragging, and a pinned text would freeze the field mid-gesture.
pub(crate) fn slot_showing(row: &SlotRow, draft: Option<&str>, notation: Notation) -> FieldShowing {
    let writing = FieldWriting::of(row.dimension, row.unit, notation);
    let number = match props::shown_value(
        writing.unit,
        match row.value {
            Ok(value) => value.as_f64(),
            Err(_) => 0.0,
        },
    ) {
        // **A driven slot keeps its field whatever its value reads
        // as.** It shows its reading as text ([`props::field_text`],
        // which says `no … reading` where the working notation cannot
        // name the value), and the field under that text is its door
        // to the expression. The number the field holds is never
        // written: a drag or a typed number over a driven slot is
        // refused by the session's driven-slot guard, so a zero held
        // there lands nowhere — the same zero a slot that did not
        // evaluate holds under its text.
        Err(_) if row.driver.is_driven() => Ok(0.0),
        number => number,
    };
    let (text, source) = match draft {
        Some(draft) => (Some(draft.to_owned()), None),
        None if row.driver.is_driven() || row.value.is_err() => (
            Some(props::field_text(row, notation)),
            props::field_source(row),
        ),
        None => (None, None),
    };
    FieldShowing {
        writing,
        dimension: row.dimension,
        number,
        text,
        source,
    }
}

/// **A document parameter's value field**: its number, in the unit
/// it was DECLARED in. A parameter is never driven, so there is no
/// text to show over it and no source to seed its edit with.
pub(crate) fn param_showing(row: &ParamRow, notation: Notation) -> FieldShowing {
    let writing = FieldWriting::of(row.dimension, row.unit, notation);
    FieldShowing {
        writing,
        dimension: row.dimension,
        number: props::shown_value(writing.unit, row.value.as_f64()),
        text: None,
        source: None,
    }
}

/// **The two doors a parameter's field has**: a bare number is a
/// value in the field's notation and nothing else moves; anything
/// else is text for `SessionOp::SetParamText`, which reads a number
/// and its notation through the one parser and refuses what is
/// neither.
pub(crate) fn param_doors(
    name: &ParamName,
) -> FieldVocabulary<impl Fn(SlotValue) -> SessionOp, impl Fn(String) -> SessionOp> {
    let (by_number, by_text) = (name.clone(), name.clone());
    FieldVocabulary {
        number: move |value| SessionOp::SetParam {
            name: by_number.clone(),
            value,
        },
        text: move |text| SessionOp::SetParamText {
            name: by_text.clone(),
            text,
        },
    }
}

/// **The two doors a slot's field has**: a bare number through
/// `SessionOp::SetSlot` (refused on a driven slot, with the
/// affordance), anything else through `SessionOp::SetSlotExpression`.
pub(crate) fn slot_doors(
    node: RecipeNodeId,
    slot: SlotId,
) -> FieldVocabulary<impl Fn(SlotValue) -> SessionOp, impl Fn(String) -> SessionOp> {
    FieldVocabulary {
        number: move |value| SessionOp::SetSlot { node, slot, value },
        text: move |text| SessionOp::SetSlotExpression { node, slot, text },
    }
}

/// **What a slot has to SAY, under its row**: the fault a slot that
/// did not evaluate carries, a driven slot's expression
/// (`label = source`, which its field does not show — [`slot_showing`]),
/// the expression-driven refusal's affordance and its edit doors, and
/// the range `reading` when one has been taken for this field.
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
    notation: Notation,
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
        if let Some(source) = &row.source {
            crate::widgets::message_toned(
                ui,
                format!("{} {} {source}", row.slot.label(), props::DRIVEN),
                theme,
                Tone::Advisory,
            );
        }
        crate::widgets::message_toned(
            ui,
            format!(
                "{}: {}",
                row.slot.label(),
                Refusal::affordance(params, row.slot, row.value.as_ref().ok().copied(), notation,)
            ),
            theme,
            Tone::Advisory,
        );
        if !params.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for name in params {
                    if crate::widgets::message_link(ui, format!("edit {}", name.as_str())).clicked()
                    {
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

/// **The hide toggle, offered exactly where `SetInstanceHidden` would
/// accept it.** `addressable` is the admission test that op runs
/// (`display::display_check`); where it refuses, the checkbox is drawn
/// disabled and the refusal's own sentence stands under it, visible
/// rather than behind a hover.
///
/// Answers whether the toggle was changed — which a refused toggle
/// never is.
fn hide_toggle(
    ui: &mut egui::Ui,
    theme: &Theme,
    addressable: &Result<(), crate::display::AdmissionFault>,
    shown: &mut bool,
) -> bool {
    let toggle = ui.add_enabled(
        addressable.is_ok(),
        egui::Checkbox::new(shown, "shown in viewport"),
    );
    if let Err(fault) = addressable {
        // Advisory: past the section's own kind gate the one fault
        // left is geometry fused into a drawn root with another
        // instance's, which is what the document says and nothing a
        // reader got wrong.
        crate::widgets::message_toned(ui, fault.to_string(), theme, Tone::Advisory);
    }
    toggle.changed()
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

    use super::{bounds_notes, exists_notice, slot_notes, slot_showing};
    use crate::pane::headless::{assert_inside, assert_own_lines, assert_under, drawn_in, find};
    use crate::props::{Notation, SlotDriver, SlotFault, SlotRow, SlotValue};
    use crate::session::Refusal;
    use crate::theme::Theme;

    /// A narrow pane, and wider than `crate::widgets::message_floor`,
    /// so these rows read the region and not the floor.
    const REGION: f32 = 260.0;

    fn param(name: &'static str) -> ParamName {
        ParamName::from_static(name)
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
        let door = format!("edit {}", name.as_str());
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
            slot_notes(ui, &Theme::DEFAULT, &row, None, Notation::DEFAULT);
        });
        let affordance = find(
            &painted,
            &format!(
                "{}: {}",
                SlotId::Distance.label(),
                Refusal::affordance(&params, SlotId::Distance, Some(value), Notation::DEFAULT)
            ),
        );
        assert_own_lines(region, affordance);
        for name in &params {
            let door = find(&painted, &format!("edit {}", name.as_str()));
            assert_inside(region, door);
            assert_under(affordance, door);
        }
    }

    /// **A driven slot's source is said under its row, whole and
    /// inside the pane** — the field does not show it
    /// (`app::properties_pane_tests` holds the row itself).
    #[test]
    fn a_driven_slots_source_is_said_under_its_row_inside_the_pane() {
        let source = "outer_enclosure_wall_thickness * 2 + gasket_compression_allowance";
        let value = SlotValue::of(Dimension::Length, 0.004).expect("a finite length");
        let row = SlotRow {
            source: Some(source.to_owned()),
            ..distance_row(
                SlotDriver::Expression {
                    params: vec![param("outer_enclosure_wall_thickness")],
                },
                Ok(value),
            )
        };
        let (region, painted) = drawn_in(REGION, |ui| {
            slot_notes(ui, &Theme::DEFAULT, &row, None, Notation::DEFAULT);
        });
        let quoted = find(
            &painted,
            &format!(
                "{} {} {source}",
                SlotId::Distance.label(),
                crate::props::DRIVEN
            ),
        );
        assert_own_lines(region, quoted);
    }

    /// **What a slot field shows, arm by arm**: a driven slot its value
    /// with its edit opening on the source; a held draft the draft, as
    /// typed, with nothing to seed — over a driven slot and a literal
    /// alike.
    #[test]
    fn a_slot_field_shows_its_reading_or_its_draft() {
        let value = SlotValue::of(Dimension::Length, 0.004).expect("a finite length");
        let driven = SlotRow {
            source: Some("thickness * 2".to_owned()),
            ..distance_row(
                SlotDriver::Expression {
                    params: vec![param("thickness")],
                },
                Ok(value),
            )
        };
        let showing = slot_showing(&driven, None, Notation::DEFAULT);
        assert_eq!(showing.text.as_deref(), Some("= 0.004 m"));
        assert_eq!(showing.source.as_deref(), Some("thickness * 2"));

        let literal = SlotRow {
            unit: Some(pncad::quantity::MM.def()),
            source: Some("4 mm".to_owned()),
            ..distance_row(SlotDriver::Literal, Ok(value))
        };
        assert_eq!(
            slot_showing(&literal, None, Notation::DEFAULT).text,
            None,
            "egui formats it"
        );
        for row in [&driven, &literal] {
            let showing = slot_showing(row, Some("thickness * undeclared"), Notation::DEFAULT);
            assert_eq!(
                showing.text.as_deref(),
                Some("thickness * undeclared"),
                "the draft is shown as typed"
            );
            assert_eq!(showing.source, None, "and is what the edit opens on");
        }
    }

    #[test]
    fn a_slots_fault_is_said_under_its_row_inside_the_pane() {
        let row = distance_row(SlotDriver::Literal, Err(SlotFault::NoExpression));
        let (region, painted) = drawn_in(REGION, |ui| {
            slot_notes(ui, &Theme::DEFAULT, &row, None, Notation::DEFAULT);
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

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use std::cell::Cell;

    use super::hide_toggle;
    use crate::app::ViewerBehavior;
    use crate::display::AdmissionFault;
    use crate::pane::headless::painted_after_clicking;
    use crate::props::{Notation, SlotDriver, SlotFault, SlotRow, SlotValue};
    use crate::session::Refusal;
    use crate::theme::Theme;
    use pncad::document::{Dimension, ParamName, RecipeNodeId, SlotId};

    const NODE: RecipeNodeId = RecipeNodeId(4);

    fn thickness() -> ParamName {
        ParamName::from_static("thickness")
    }

    /// One extrude distance row, driven or not, with the value the
    /// document's parameters give it.
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

    /// **The disabled range button reads the refused operation's own
    /// value** — the variant and its payload, not a sentence about it.
    ///
    /// `guard_driven` builds `DrivenByExpression` from the same row's
    /// driver and value, so the fields asserted here are the ones the
    /// click's refusal carries; the rendering follows from the value
    /// rather than being a second composition beside it. Both are
    /// asserted: a control that started minting words again would keep
    /// the variant and lose the sentence.
    #[test]
    fn a_driven_slots_range_button_reads_the_refusal_the_probe_would_give() {
        let current = SlotValue::Continuous(0.004);
        let millimetres = Notation {
            length: pncad::quantity::MM,
            ..Notation::DEFAULT
        };
        let row = distance_row(
            SlotDriver::Expression {
                params: vec![thickness()],
            },
            Ok(current),
        );
        match ViewerBehavior::probe_refusal(NODE, &row, millimetres) {
            Some(Refusal::DrivenByExpression {
                node,
                slot,
                ref params,
                current: carried,
                notation,
            }) => {
                assert_eq!(node, NODE);
                assert_eq!(slot, SlotId::Distance);
                assert_eq!(params, &vec![thickness()], "what to edit instead");
                assert_eq!(carried, Some(current));
                assert_eq!(notation, millimetres, "and the notation it reads in");
            }
            ref other => panic!("expected the driven refusal, got {other:?}"),
        }
        let rendered = ViewerBehavior::probe_refusal(NODE, &row, millimetres)
            .expect("a driven slot is refused the probe")
            .to_string();
        assert_eq!(
            rendered,
            Refusal::affordance(&[thickness()], SlotId::Distance, Some(current), millimetres),
            "and it renders as the ratified affordance, from its one home"
        );
        // The mapping itself, planted: the words a reader gets for this
        // row. The coupling above holds under any rewording of the one
        // home; this line does not.
        assert_eq!(
            rendered,
            "driven by an expression over thickness (currently 4 mm) — edit the expression?"
        );
    }

    /// The fault a fused instance's display doors refuse with.
    fn fused() -> AdmissionFault {
        AdmissionFault::FusedGeometry {
            instance: RecipeNodeId(0),
            root: RecipeNodeId(2),
            others: vec![RecipeNodeId(1)],
        }
    }

    /// **A refused hide toggle is drawn, cannot be flipped, and carries
    /// the door's own sentence under it** — the panel half of the fused
    /// instance's repair, drawn headless.
    ///
    /// The runtime value that makes it false is a toggle gated on
    /// anything weaker than the display doors' own test: clicked here,
    /// it would report a change the op then refuses.
    #[test]
    fn a_refused_hide_toggle_is_disabled_and_says_why() {
        let changed = Cell::new(false);
        let painted = painted_after_clicking("shown in viewport", |ui| {
            let mut shown = true;
            if hide_toggle(ui, &Theme::DEFAULT, &Err(fused()), &mut shown) {
                changed.set(true);
            }
        });
        assert!(!changed.get(), "a refused toggle is not the user's to flip");
        // Planted, not compared with another reading of the fault.
        assert!(
            painted.contains(
                "instance 0's geometry is fused into node 2 together with instance(s) 1 — \
                 a display operation cannot address it separately"
            ),
            "{painted}"
        );
    }

    /// **The same click on an addressable toggle DOES flip it** — the
    /// row that keeps the one above from passing because the harness
    /// missed the checkbox, and that no sentence is owed where the op
    /// would accept.
    #[test]
    fn an_addressable_hide_toggle_flips_and_says_nothing() {
        let changed = Cell::new(false);
        let painted = painted_after_clicking("shown in viewport", |ui| {
            let mut shown = true;
            if hide_toggle(ui, &Theme::DEFAULT, &Ok(()), &mut shown) {
                changed.set(true);
            }
        });
        assert!(changed.get(), "the click reached the checkbox");
        assert!(!painted.contains("fused"), "{painted}");
    }

    /// **A slot the user can write is offered the probe**, with nothing
    /// owed: `probe_bounds` would accept the click.
    #[test]
    fn a_literal_slot_is_offered_the_probe() {
        assert!(
            ViewerBehavior::probe_refusal(
                NODE,
                &distance_row(SlotDriver::Literal, Ok(SlotValue::Continuous(0.008))),
                Notation::DEFAULT,
            )
            .is_none()
        );
    }

    /// **The driver decides the arm, and a driven row with no value
    /// still gets the refusal** — naming what to edit instead is most
    /// of what that reader needs, and `current` goes `None` rather
    /// than the whole affordance going away.
    ///
    /// This is also the one shape in which a row reaches the panel with
    /// an `Err` value and no `EvalError`: `props::slot_row` reports a
    /// slot its node lists and carries no expression for as
    /// `SlotFault::NoExpression`, and classifies it as driven with an
    /// empty parameter list, which is the refusing direction.
    #[test]
    fn a_driven_slot_that_did_not_evaluate_still_gets_the_refusal() {
        let row = distance_row(
            SlotDriver::Expression {
                params: vec![thickness()],
            },
            Err(SlotFault::NoExpression),
        );
        let refusal = ViewerBehavior::probe_refusal(NODE, &row, Notation::DEFAULT)
            .expect("a driven slot is refused the probe");
        assert!(
            matches!(refusal, Refusal::DrivenByExpression { current: None, .. }),
            "no current value to name: {refusal:?}"
        );
        assert_eq!(
            refusal.to_string(),
            Refusal::affordance(&[thickness()], SlotId::Distance, None, Notation::DEFAULT)
        );
        assert!(refusal.to_string().contains("thickness"));
    }
}

/// **How loud the selection's verdict is drawn**, read off the paint
/// and held against fixed colours ([`crate::pane::headless::Voices`]),
/// through [`standing_verdict`] — the function the header calls — so
/// a literal tone put back at any of its arms, or a swapped mapping
/// anywhere between [`Standing::tone`] and the glyphs, turns a row red.
#[cfg(test)]
mod verdict_tests {
    use editor_core::RecipeEditRef;
    use pncad::document::{NodeStanding, ParamName, RecipeNodeId};
    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
    use pncad::select::{Resolution, ResolutionFailure, ResolveError, ResolveIndeterminate};

    use super::standing_verdict;
    use crate::pane::headless::{Landed, Voices, find, find_opening, landed_voiced};
    use crate::session::{EdgeSelection, FaceSelection, Standing};
    use crate::theme::Theme;

    fn name(kind: EntityKind) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId(1),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        }
    }

    fn face(resolution: Option<Resolution>) -> Standing {
        Standing::Face {
            face: FaceSelection {
                name: name(EntityKind::Face),
                node: RecipeNodeId(2),
                body: 0,
            },
            resolution: resolution.map(Box::new),
        }
    }

    fn vanished(offers: Vec<StableName>) -> Resolution {
        Resolution::Failed(ResolutionFailure {
            error: ResolveError::NodeGone {
                name: name(EntityKind::Face),
                edit: RecipeEditRef::NodeDeleted {
                    node: RecipeNodeId(1),
                },
            },
            offers,
        })
    }

    /// What [`standing_verdict`] painted for `standing`.
    fn drawn(standing: &Standing) -> (Vec<Landed>, Voices) {
        landed_voiced(&Theme::DEFAULT, |ui, theme| {
            standing_verdict(ui, theme, standing)
        })
    }

    /// **A name that no longer resolves is a verdict to act on**, so
    /// it is drawn in the actionable colour — and the rebind count
    /// under it is secondary text, weak.
    #[test]
    fn a_vanished_faces_verdict_is_drawn_loud_and_its_offer_count_weak() {
        let (painted, voices) = drawn(&face(Some(vanished(vec![name(EntityKind::Face)]))));
        assert_eq!(
            find_opening(&painted, "this face is gone: ").ink,
            Some(voices.actionable)
        );
        assert_eq!(
            find(&painted, "1 rebind candidate(s) offered").ink,
            Some(voices.weak)
        );
    }

    /// An entity the evaluation could not answer for is a verdict to
    /// act on too — the edge arm, and its noun off its own arm.
    #[test]
    fn an_indeterminate_edges_verdict_is_drawn_loud() {
        let standing = Standing::Edge {
            edge: EdgeSelection {
                name: name(EntityKind::Edge),
                node: RecipeNodeId(2),
                body: 0,
            },
            resolution: Some(Box::new(Resolution::Indeterminate(ResolveIndeterminate {
                standing: NodeStanding::Failed {
                    node: RecipeNodeId(1),
                },
            }))),
        };
        let (painted, voices) = drawn(&standing);
        assert_eq!(
            find_opening(&painted, "this edge cannot be resolved right now: ").ink,
            Some(voices.actionable)
        );
    }

    /// **No evaluation yet is not a verdict about the pick**: it is
    /// said, quietly.
    #[test]
    fn a_pick_with_no_evaluation_behind_it_is_said_weak() {
        let (painted, voices) = drawn(&face(None));
        assert_eq!(
            find(&painted, "no evaluation yet to resolve this against").ink,
            Some(voices.weak)
        );
    }

    /// **A deleted node's one word is loud**, beside its number.
    #[test]
    fn a_deleted_nodes_verdict_is_drawn_loud() {
        let (painted, voices) = drawn(&Standing::Node {
            node: RecipeNodeId(3),
            present: false,
        });
        assert_eq!(find(&painted, "deleted").ink, Some(voices.actionable));
    }

    /// **An undeclared parameter is said once, loud** — the only line
    /// the pane draws for it (`properties_ui`'s `Param` arm draws none).
    #[test]
    fn an_undeclared_parameters_verdict_is_drawn_loud() {
        let (painted, voices) = drawn(&Standing::Param {
            name: ParamName::from_static("width"),
            present: false,
        });
        assert_eq!(
            find(&painted, "parameter width is no longer declared").ink,
            Some(voices.actionable)
        );
    }

    /// **A selection that still denotes has no verdict**, so nothing
    /// is said at all — not a quiet "fine".
    #[test]
    fn a_standing_that_still_denotes_says_nothing() {
        for standing in [
            Standing::Empty,
            Standing::Node {
                node: RecipeNodeId(3),
                present: true,
            },
            Standing::Param {
                name: ParamName::from_static("width"),
                present: true,
            },
        ] {
            let (painted, _) = drawn(&standing);
            assert!(
                painted.is_empty(),
                "{standing:?} painted {:?}",
                painted.len()
            );
        }
    }
}
