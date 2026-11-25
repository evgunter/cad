//! The Properties pane: what the current selection is, and the slot,
//! variable and instance fields that edit it.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{
    Axis3, Dimension, Doc, Evaluation, Frame, HeldNodes, ProfileProgram, RecipeNodeId, Said,
    Sentence, SlotId, VarId, VarName,
};
use pncad::quantity::UnitDef;
use pncad::select::{AboutReference, Resolution};

use crate::app::{ViewerBehavior, indeterminate_wording, toned};
use crate::display::free_move_check;
use crate::drafts::NameDraft;
use crate::forms::FieldWriting;
use crate::frame::Tone;
use crate::props::{self, Notation, SlotDriver, SlotGroup, SlotRow, SlotValue, VariableRow};
use crate::session::{
    NodeKindWanted, RangeTarget, Refusal, Selection, SessionOp, Standing, ValueGestureName, admits,
};
use crate::theme::Theme;
use crate::tree::PickedName;
use crate::widgets::{
    CANCEL, FieldShowing, FieldVocabulary, ProbeOps, UNIT_PICKER_WIDTH, commit_row, delete_button,
    form_door, free_move_gesture, named_field, notation_picker, pick_unit, unit_choices,
    value_field_ops, value_gesture, vec3_row_ops,
};

/// What the door that opens the add-variable form says.
pub(crate) const NEW_VARIABLE: &str = "New variable…";

/// What the product door's button says ([`SessionOp::AddToProduct`]).
pub(crate) use crate::recourse::ADD_TO_THE_PRODUCT;

/// What the button taking a copy out of the product says.
pub(crate) const TAKE_OUT_OF_THE_PRODUCT: &str = "Take out of the product";

/// What the add-variable form's commit button says: what it makes, so
/// it reads apart from every other commit on screen.
pub(crate) const NEW_VARIABLE_COMMIT: &str = "Create variable";

impl ViewerBehavior<'_> {
    /// The property panel.
    pub(crate) fn properties_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Properties");
        ui.separator();
        self.create_ui(ui);
        let standing = self.session.standing();
        self.standing_ui(ui, &standing);
        if let Some(node) = self.session.selection().node() {
            self.display_ui(ui, node);
        }
        self.product_ui(ui);
        self.placement_ui(ui);
        // The forms whose op takes the selection, under its header and
        // above its rows, where a reader who picked it looks first:
        // nothing at all unless the selection is a profile.
        self.sweep_ui(ui);
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
                let doc = self.session.doc();
                let related = crate::relations::relations(
                    doc,
                    &doc.read_index(),
                    node,
                    self.session.part_files(),
                );
                relation_rows(ui, doc, Direction::Reads, &related.reads, self.ops);
                relation_rows(ui, doc, Direction::ReadBy, &related.read_by, self.ops);
            }
            // Slot rows for the feature that MADE the picked entity —
            // the node `slot_groups` itself answered for, so the rows
            // shown and the node an edit lands on are one answer. A
            // pick is a way of reaching that feature, which is what
            // G3's click-to-select is for, and an edge reaches it the
            // same way a face does.
            Selection::Face(_) | Selection::Edge(_) | Selection::Vertex(_) => {
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
            Selection::Variable(var) => {
                // **The previewed document here, deliberately**: this
                // row's field IS the drag, so it must show the value
                // the gesture is previewing. What decides whether the
                // row is drawn at all — the declaration — is the same
                // in both documents, because the only edit a preview
                // applies to a variable is a value write
                // (`props::variable_edit`), which cannot declare or
                // undeclare one.
                if let Some(row) = crate::props::variable_rows(self.session.doc())
                    .into_iter()
                    .find(|row| row.var == var)
                {
                    // The dimension in the common noun editor-core's
                    // `Display` spells, never the variant identifier:
                    // a label a person reads is prose.
                    crate::widgets::message(
                        ui,
                        format!(
                            "variable {} ({})",
                            crate::tree::spelled_var(&row.label),
                            row.dimension
                        ),
                    );
                    ui.horizontal(|ui| {
                        value_field_ops(
                            ui,
                            variable_showing(&row, *self.notation),
                            value_gesture(ValueGestureName::Variable(var)),
                            variable_doors(var),
                            self.ops,
                            self.notices,
                        );
                        // **The unit is the picker's to say.** A
                        // variable's notation is a fact the document
                        // stores and an edit changes
                        // (`SessionOp::SetVariableUnit`, over
                        // `DocEdit::SetVarUnit`), so the row says
                        // it the way a slot row does: with the control
                        // that changes it. The dimensionless row and a
                        // `Count` have no notation to offer and draw
                        // nothing.
                        self.variable_unit_ui(ui, &row);
                    });
                    self.variable_range_ui(ui, &row);
                } else if let Some(row) = crate::props::defined_rows(self.session.doc())
                    .into_iter()
                    .find(|row| row.var == var)
                {
                    // A defined variable's field shows its formula and
                    // takes text as a free one's does: a formula
                    // redefines it, a value makes it free again. It
                    // holds no value of its own, so a drag is the value
                    // door's to refuse.
                    crate::widgets::message(
                        ui,
                        format!(
                            "variable {} ({})",
                            crate::tree::spelled_var(&row.label),
                            row.dimension
                        ),
                    );
                    value_field_ops(
                        ui,
                        defined_showing(&row, *self.notation),
                        value_gesture(ValueGestureName::Variable(var)),
                        variable_doors(var),
                        self.ops,
                        self.notices,
                    );
                    for line in reading_notes(
                        &crate::tree::spelled_var(&row.label),
                        &row.value,
                        row.dimension,
                        *self.notation,
                    ) {
                        crate::widgets::message_toned(ui, line, &self.theme, Tone::Advisory);
                    }
                } else if crate::props::index_count_row(self.session.doc(), var).is_some() {
                    crate::widgets::message(
                        ui,
                        format!(
                            "index {}",
                            crate::tree::spelled_var(&self.session.doc().spoken_var(var))
                        ),
                    );
                    self.index_count_ui(ui, var);
                }
                // No row is an undeclared variable, and nothing is
                // drawn for it here: the header above has said so
                // ([`standing_verdict`]), loud, and this panel says a
                // fact once — the rule `failure_lines` keeps for a
                // failed tree row and `display_ui` keeps for a
                // vanished instance. The header's `present` reads the
                // same document this lookup does
                // (`DocSession::standing`), so the two are one answer.
            }
        }
        ui.separator();
        ui.label("variables");
        for row in crate::props::variable_rows(self.session.doc()) {
            // A name the user authored, so nothing bounds its width.
            if crate::widgets::message_link(ui, variable_line(&row, *self.notation)).clicked() {
                self.ops
                    .push(SessionOp::Select(Selection::Variable(row.var)));
            }
        }
        for row in crate::props::defined_rows(self.session.doc()) {
            if crate::widgets::message_link(ui, defined_line(&row, *self.notation)).clicked() {
                self.ops
                    .push(SessionOp::Select(Selection::Variable(row.var)));
            }
            // Why it reads nothing, under its line — except while it is
            // selected, when its own row above says it (`reading_notes`)
            // and this pane says a fact once.
            if self.session.selection() != &Selection::Variable(row.var) {
                for line in reading_faults(&crate::tree::spelled_var(&row.label), &row.value) {
                    crate::widgets::message_toned(ui, line, &self.theme, Tone::Advisory);
                }
            }
        }
        for row in crate::props::source_rows(self.session.doc()) {
            self.source_row_ui(ui, &row);
        }
        self.add_variable_ui(ui);
    }

    /// **A named source's row** (VARIABLES VR4): `bracket = B`, and its
    /// body field, set by picking — the button re-points the source onto
    /// the body selected in the tree or the picture
    /// ([`crate::session::DocSession::repoint_button`]), disabled with the
    /// reason where the offer would withhold it.
    fn source_row_ui(&mut self, ui: &mut egui::Ui, row: &crate::props::SourceRow) {
        let button = self
            .session
            .repoint_button(row.var, self.session.selection().node());
        let plural = if row.copies == 1 { "y" } else { "ies" };
        ui.horizontal(|ui| {
            if crate::widgets::message_link(
                ui,
                format!(
                    "{} = {} ({} cop{plural})",
                    crate::tree::spelled_var(&row.label),
                    crate::tree::spelled_var(&row.body),
                    row.copies
                ),
            )
            .clicked()
            {
                self.ops
                    .push(SessionOp::Select(Selection::Variable(row.var)));
            }
            crate::widgets::repoint_button(ui, button, self.ops);
        });
    }

    /// **A feature's editing rows**: its slot rows — or, for a
    /// profile, the profile editor
    /// ([`ViewerBehavior::edit_profile_ui`]), whose fields ARE its
    /// slots' fields. Only a profile the editor cannot hold shows its
    /// refusal and the slot rows instead.
    fn feature_rows_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, groups: &[SlotGroup]) {
        let profile = admits(self.session.committed_doc(), node, NodeKindWanted::Profile);
        if profile && self.edit_profile_ui(ui, node) {
            return;
        }
        for group in groups {
            self.slot_group_ui(ui, node, group);
        }
        for index in crate::props::indices_read(self.session.doc(), node) {
            self.index_count_ui(ui, index);
        }
    }

    /// **An index's count row** (D10 `[repetition]`): the one widget
    /// that edits it, drawn on the index variable's own row and on the
    /// rows of each node that reads it, a pattern's placement among
    /// them. A number or a formula goes through the variable's doors,
    /// which write an index's count by `props::index_count_edit`.
    pub(crate) fn index_count_ui(&mut self, ui: &mut egui::Ui, index: VarId) {
        let Some(row) = crate::props::index_count_row(self.session.doc(), index) else {
            return;
        };
        ui.horizontal(|ui| {
            ui.label(format!("count of {}", crate::tree::spelled_var(&row.label)));
            value_field_ops(
                ui,
                defined_showing(&row, *self.notation),
                value_gesture(ValueGestureName::Variable(index)),
                variable_doors(index),
                self.ops,
                self.notices,
            );
        });
        for line in reading_notes(
            &format!("the count of {}", crate::tree::spelled_var(&row.label)),
            &row.value,
            row.dimension,
            *self.notation,
        ) {
            crate::widgets::message_toned(ui, line, &self.theme, Tone::Advisory);
        }
    }

    /// The create half of the document-variables section: name,
    /// dimension, value, the NOTATION to write it in, one
    /// [`SessionOp::DeclareVar`] on commit.
    ///
    /// **The unit is the working notation's, not this form's.** A
    /// length picked here is [`Notation::length`], the one every
    /// creation form in the crate writes its lengths in and every value
    /// nobody wrote reads in ([`props::Notation`]). The panel's
    /// pickers are per literal for the opposite reason, and a
    /// variable's DECLARED notation is one of those: it is changed
    /// afterwards at its own row ([`ViewerBehavior::variable_unit_ui`]),
    /// not here.
    ///
    /// Two deliberate frictions, both refusals-in-advance. The
    /// dimension starts UNPICKED and the commit waits for it — the offer
    /// path arrives here from an expression whose context does not
    /// determine the new variable's dimension, and a silent default
    /// would be a guess. And a name that is already declared shows the
    /// already-exists sentence ([`Refusal::exists_wording`]) with the
    /// edit door offered, before the click ever reaches the declare's
    /// own refusal of a taken name (`EditError::VarNameTaken`).
    pub(crate) fn add_variable_ui(&mut self, ui: &mut egui::Ui) {
        if !form_door(ui, NEW_VARIABLE, &mut self.drafts.new_variable_open) {
            return;
        }
        // The offer from an unknown-variable parse refusal, shown
        // while the name field still says the offered name.
        if let Some(offered) = self.drafts.new_variable_offer.clone() {
            if offered.as_str() == self.drafts.new_variable_name.trim() {
                crate::widgets::message_toned(
                    ui,
                    Refusal::offer_wording(&offered),
                    &self.theme,
                    Tone::Advisory,
                );
            } else {
                // The user typed past the offer; it is stale.
                self.drafts.new_variable_offer = None;
            }
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.drafts.new_variable_name)
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
                    &mut self.drafts.new_variable_dimension,
                    Some(dimension),
                    dimension.to_string(),
                );
            }
            // **The form authors in the notation the picker says**, and
            // the draft behind the field stays canonical whatever it
            // says (`widgets::named_field`). With no dimension picked
            // yet the commit is refused, and the field is a bare number
            // until one is.
            let writing = self
                .drafts
                .new_variable_dimension
                .map_or(FieldWriting::unpicked(), |dimension| {
                    FieldWriting::of(dimension, None, *self.notation)
                });
            named_field(ui, "", writing, &mut self.drafts.new_variable_value);
            // Drawn AFTER the field it governs, which is the forms'
            // rule (`widgets::length_picker`): the pick is an input
            // event, so the field it re-writes is next frame's.
            if let Some(dimension) = self.drafts.new_variable_dimension {
                notation_picker(ui, "add_variable", dimension, self.notation);
            }
        });
        // The draft text is offered to the one door that decides what
        // a variable name is; a refused text leaves the control
        // disabled, and the refusal's own sentence is said under it.
        let typed = self.drafts.new_variable_name.trim().to_owned();
        let named = VarName::new(typed.as_str());
        let name = named.as_ref().ok().cloned();
        // The declare door applies to `committed_doc()`, so the notice
        // ahead of the click asks it too: a notice drawn from the
        // previewed document would be answering about a document the
        // door will not see.
        let committed = self.session.committed_doc();
        let existing = name
            .as_ref()
            .and_then(|name| crate::props::named_variable(committed, name));
        if let (Some(name), Some((var, dimension))) = (&name, existing) {
            if exists_notice(ui, &self.theme, name, dimension) {
                self.ops.push(SessionOp::Select(Selection::Variable(var)));
            }
            if ui.button(CANCEL).clicked() {
                self.drafts.close_new_variable();
            }
            return;
        }
        // **The commit's gate, evaluated once**: whether the button is
        // live, the reason said under the row while it is not, and the
        // declaration a click pushes are all this one answer, so the
        // form cannot offer a click that does nothing. The name first,
        // as the form reads; then the dimension; then whether the
        // dimension can carry the draft value — a `Count` declared
        // from a field holding `NaN` would otherwise be created holding
        // zero, a value nobody authored, and `SlotValue::of` is the one
        // door that decides it. A refusal is said in its door's words.
        let gate = match named {
            Err(_) if typed.is_empty() => Err(("name the variable".to_owned(), Tone::Advisory)),
            Err(fault) => Err((fault.to_string(), Tone::Actionable)),
            Ok(name) => self
                .drafts
                .new_variable_dimension
                .ok_or_else(|| ("pick a dimension first".to_owned(), Tone::Advisory))
                .and_then(|dimension| {
                    SlotValue::of(dimension, self.drafts.new_variable_value)
                        .map(|value| (name, dimension, value))
                        .map_err(|fault| (fault.to_string(), Tone::Actionable))
                }),
        };
        if let Err((words, tone)) = &gate {
            crate::widgets::message_toned(ui, words, &self.theme, *tone);
        }
        let closing = commit_row(
            ui,
            NEW_VARIABLE_COMMIT,
            gate.as_ref().err().map(|(words, _)| words.as_str()),
            |_| {},
        );
        if closing.cancel {
            self.drafts.close_new_variable();
        }
        // The form closes when the document accepts the declaration
        // (`Drafts::accepted`), so a refused one keeps what was typed.
        if closing.commit
            && let Ok((name, dimension, value)) = gate
        {
            self.ops.push(SessionOp::DeclareVar {
                name,
                value: crate::props::doc_variable(
                    dimension,
                    value,
                    FieldWriting::of(dimension, None, *self.notation).unit,
                ),
            });
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
        self.drafts.rename_shown_for(match standing {
            Standing::Node {
                node,
                present: true,
            } => Some(*node),
            _ => None,
        });
        let landed = self.session.landed_pair();
        let said = self.session.selection_said();
        match standing {
            Standing::Empty | Standing::Variable { .. } => {}
            Standing::Node { node, present } => {
                ui.horizontal(|ui| {
                    let heading = selected_node_heading(
                        landed.map(|(doc, _)| doc),
                        self.session.part_files(),
                        said,
                        *node,
                    );
                    let heading = crate::widgets::sentence(ui, &heading, self.session.doc(), None);
                    self.ops.extend(heading.map(SessionOp::Select));
                    if *present && delete_button(ui, self.session, *node) {
                        self.ops.push(SessionOp::DeleteNode { node: *node });
                    }
                    // Beside the node's name, which is the node it is about.
                    standing_verdict(ui, &self.theme, standing, landed, said);
                });
                if *present {
                    self.label_ui(ui, *node);
                }
                return;
            }
            Standing::Face { face, .. } => {
                let picked = PickedName::face(face);
                self.entity_header_ui(ui, &picked, standing);
            }
            Standing::Edge { edge, .. } => {
                let picked = PickedName::edge(edge);
                self.entity_header_ui(ui, &picked, standing);
            }
            Standing::Vertex { vertex, .. } => {
                let picked = PickedName::vertex(vertex);
                self.entity_header_ui(ui, &picked, standing);
            }
        }
        standing_verdict(ui, &self.theme, standing, landed, said);
    }

    /// **The rename field** (DESIGN.md Band 1, "Node labels"): the
    /// node's label, editable in place. Leaving the field commits what
    /// was typed as one [`SessionOp::SetLabel`] — blank clears the
    /// label, and the label the node already has is no edit
    /// (`DocSession::writes_nothing`). A text the label rule refuses
    /// is said on the status line, and the field goes back to the
    /// node's label.
    fn label_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId) {
        let mut text = match &self.drafts.label_text {
            Some((typed_for, typed)) if *typed_for == node => typed.clone(),
            _ => self
                .session
                .doc()
                .label(node)
                .map(|label| label.as_str().to_owned())
                .unwrap_or_default(),
        };
        ui.horizontal(|ui| {
            ui.label("label");
            let response = ui.text_edit_singleline(&mut text);
            if response.changed() {
                self.drafts.label_text = Some((node, text));
            }
            if response.lost_focus()
                && let Some((typed_for, typed)) = self.drafts.label_text.take()
                && typed_for == node
            {
                match crate::drafts::label_typed(&typed) {
                    Ok(label) => self.ops.push(SessionOp::SetLabel { node, label }),
                    Err(fault) => self.notices.push(crate::frame::tool_news(
                        &crate::tools::FormKind::Label.refuses(&fault),
                    )),
                }
            }
        });
    }

    /// A picked entity's header line ([`entity_header`]): the entity as
    /// the chrome says a pick, and the delete the feature that made it
    /// offers. Its verdict is [`standing_verdict`]'s, on the lines
    /// under it.
    fn entity_header_ui(
        &mut self,
        ui: &mut egui::Ui,
        picked: &PickedName<'_>,
        standing: &Standing,
    ) {
        ui.horizontal(|ui| {
            let by = crate::frame::landed_speaker(self.session.landed_pair())
                .or_held(self.session.selection_said());
            let (label, feature) = entity_header(picked, by);
            let label = crate::widgets::sentence(ui, &label, self.session.doc(), None);
            self.ops.extend(label.map(SessionOp::Select));
            if standing.live() && delete_button(ui, self.session, feature) {
                self.ops.push(SessionOp::DeleteNode { node: feature });
            }
        });
    }

    /// The selected body's display controls: the hide toggle and the
    /// free-move probe. Draws nothing for a node the document does not
    /// admit display state on — one that builds no body, or an id the
    /// document no longer holds.
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
    /// The body's toggle pushes [`SessionOp::SetBodyOn`], whose door
    /// runs [`crate::display::body_check`] — the test that gates this
    /// section — so the toggle is offered exactly where the op accepts.
    pub(crate) fn display_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId) {
        // **The COMMITTED document, because that is the one the doors
        // read.** `set_body_on` and `begin_free_move` are handed
        // `history.doc()`, and both ops are permitted during a value
        // gesture, so a panel reading the previewed document would be
        // answering a different question from the click it is standing
        // in for. Reading the same document makes the agreement
        // structural instead of circumstantial.
        let doc = self.session.committed_doc();
        // The fault is discarded HERE, at the party that decides the
        // two refusals are one answer, rather than by a door that
        // answered a `bool` and could not have offered anything else.
        if crate::display::body_check(doc, node).is_err() {
            return;
        }
        ui.separator();
        let heading = crate::relations::Related::node(doc, node, self.session.part_files());
        let heading = crate::widgets::sentence(ui, heading.said(), self.session.doc(), None);
        self.ops.extend(heading.map(SessionOp::Select));
        let mut on = self.toggles.body_on(node);
        if body_toggle(ui, &mut on) {
            self.ops.push(SessionOp::SetBodyOn { node, on });
        }
        // The evaluation the op reads (DI5: free-move moves a shown
        // space, and a body with no value has no picture to move).
        match free_move_check(doc, self.session.space_evaluation(), node) {
            Err(fault) => {
                // The typed ineligibility, shown where the control
                // would be — the same sentence the op would refuse
                // with. Advisory for every fault that reaches here: a
                // body in the world (the document working as written),
                // a picture not known yet (an evaluation still on its
                // way), a space no drawn body is in, neither of which
                // asks anything of the reader, or a node the evaluation
                // refused, whose failure the tree's row already reports
                // where it lives — here it only says why there is no
                // probe.
                crate::widgets::message_toned(ui, fault.to_string(), &self.theme, Tone::Advisory);
                // The recourse the sentence names, where it is one: a
                // copy placed into the world is given a space of its
                // own, which the probe then moves.
                if matches!(fault, crate::display::AdmissionFault::InTheWorld { .. })
                    && doc
                        .node(node)
                        .is_some_and(pncad::document::Node::makes_a_copy)
                    && ui.button("Into a space of its own").clicked()
                {
                    self.ops.push(SessionOp::IntoOwnSpace { placement: node });
                }
            }
            Ok(space) => {
                let current = self
                    .display
                    .moved
                    .get(&space)
                    .map_or([0.0; 3], |frame| frame.translation);
                free_move_probe(ui, node, current, *self.notation, self.ops);
            }
        }
    }

    /// **The copy the selection is on, and the steps it offers**
    /// ([`crate::frame::placement_offers`]): the way to the original
    /// body from a placed copy, and the re-point of a body's placements
    /// onto a body built on it, each beside the fact it acts on. Read
    /// off the committed document, once per change ([`crate::session::DocSession::placement_offers`]).
    fn placement_ui(&mut self, ui: &mut egui::Ui) {
        let Some(node) = self.session.selection().seat_node() else {
            return;
        };
        for offer in self.session.placement_offers(node) {
            crate::widgets::message(ui, offer.said);
            if ui.button(offer.label).clicked() {
                self.ops.push(offer.op);
            }
        }
    }

    /// **Whether the selected body is in what the document makes**, and
    /// the control that says it is or is not (D10 `[operations]`): a body
    /// the product slot reads can be taken out of it
    /// ([`SessionOp::SetProduct`]),
    /// and any other drawn body added to it through the one door
    /// ([`SessionOp::AddToProduct`]). The body is the one the selection
    /// draws ([`Selection::seat_node`]): under a viewport pick the copy
    /// the ray met, not the feature that minted the face. Read off the
    /// committed document, the one the ops read.
    fn product_ui(&mut self, ui: &mut egui::Ui) {
        let selection = self.session.selection();
        let Some(node) = selection.seat_node() else {
            return;
        };
        let body = selection.seat_body();
        let doc = self.session.committed_doc();
        let read = crate::world::body_read(doc, node, body)
            .and_then(|read| crate::world::read_var(doc, &read));
        if let Some(var) = read.filter(|var| crate::world::product_reads(doc).contains(var)) {
            crate::widgets::message(ui, "in the product");
            if ui.button(TAKE_OUT_OF_THE_PRODUCT).clicked() {
                self.ops.push(SessionOp::SetProduct {
                    bodies: crate::world::product_without(doc, var),
                });
            }
        } else if read.is_some()
            && crate::world::is_drawn(doc, node)
            && ui.button(ADD_TO_THE_PRODUCT).clicked()
        {
            self.ops.push(SessionOp::AddToProduct { node, body });
        }
    }

    /// One PANEL ROW: a scalar slot on its own, or a 3-vector's three
    /// components on one line.
    ///
    /// The grouping is [`crate::props::SlotGroup`]'s and the vocabulary's (see
    /// its docs); this function only lays it out. What the two arms
    /// share — the value field (numbers AND expressions, one widget),
    /// the gesture mapping, the driven affordance, `range?` —
    /// is called once per COMPONENT, so a component of a vector is
    /// edited by exactly the operations a stand-alone slot is.
    ///
    /// **Three lines per group at most, whatever its arity.** Folding
    /// three slots onto one line buys nothing if their doors then take
    /// three lines each, so `range?` is a single line of small
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
                    ui.label(row.label.as_str());
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
                    self.name_button(ui, node, row, "name…");
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
                // variables driving one component, and a range is one
                // field's. Each names its component's slot.
                for row in rows.iter() {
                    self.slot_notes_ui(ui, node, row);
                }
                let tagged: Vec<(&SlotRow, &str)> = rows
                    .iter()
                    .zip(Axis3::ALL)
                    .map(|(row, axis)| (row, axis.label()))
                    .collect();
                ui.horizontal(|ui| self.tagged_doors(ui, node, &tagged, SlotDoor::Range));
                ui.horizontal(|ui| self.tagged_doors(ui, node, &tagged, SlotDoor::Name));
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
    /// else — an operator, a variable, a unit — through
    /// `SessionOp::SetSlotExpression`. The panel parses nothing.
    ///
    /// The field commits on Enter or on leaving it
    /// (`update_while_editing(false)`), never per keystroke: half of
    /// `thickness * 2` is a parse refusal at best and a DIFFERENT
    /// variable at worst.
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
        // the variable row's field is this same call with its own
        // two operations. What a slot contributes is what the field
        // shows and what its edit opens on ([`slot_showing`]): a
        // number typed over a driven slot's reading is no echo of
        // anything, so it replaces the expression even when the number
        // happens to match.
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

    /// **The doors under a row of a profile editor's held fields**
    /// (`pane::profile`'s `held_field`), each tagged by its field's
    /// name: one line of `range?` and naming buttons
    /// ([`Self::tagged_doors`], the vector group's), then each slot's
    /// notes ([`Self::slot_notes_ui`]): its expression, its range
    /// reading, its offer and its naming field. Each row is the
    /// committed slot's, as the document holds it: the slot every door
    /// here acts on.
    ///
    /// **Naming and offers wait on held edits**: naming an argument's
    /// variable, or taking an offer, makes the slot read a named
    /// variable in the document, which moves the program the draft was
    /// loaded from, and the draft would reload over what it holds.
    /// While the draft is not `untouched` the name buttons are not
    /// drawn and the notes leave out the offer.
    pub(crate) fn held_slot_doors_ui(
        &mut self,
        ui: &mut egui::Ui,
        node: RecipeNodeId,
        rows: &[(SlotRow, String)],
        untouched: bool,
    ) {
        let tagged: Vec<(&SlotRow, &str)> = rows
            .iter()
            .map(|(row, name)| (row, name.as_str()))
            .collect();
        ui.horizontal(|ui| {
            self.tagged_doors(ui, node, &tagged, SlotDoor::Range);
            if untouched
                && rows
                    .iter()
                    .any(|(row, _)| self.unnamed(node, row).is_some())
            {
                self.tagged_doors(ui, node, &tagged, SlotDoor::Name);
            }
        });
        for (row, _) in rows {
            if untouched {
                self.slot_notes_ui(ui, node, row);
            } else {
                self.slot_readings_ui(ui, node, row);
            }
        }
    }

    /// **One door per component, tagged**: `range` or `name`, then a
    /// small button per row labelled by its tag — the line a vector's
    /// group and a row of held fields both draw.
    fn tagged_doors(
        &mut self,
        ui: &mut egui::Ui,
        node: RecipeNodeId,
        rows: &[(&SlotRow, &str)],
        door: SlotDoor,
    ) {
        match door {
            SlotDoor::Range => {
                ui.weak("range");
                for (row, tag) in rows {
                    self.range_button(ui, node, row, tag);
                }
            }
            SlotDoor::Name => {
                ui.weak("name");
                for (row, tag) in rows {
                    self.name_button(ui, node, row, tag);
                }
            }
        }
    }

    /// The written-unit picker for a document variable's row.
    ///
    /// **"How do I want this number written" is an edit**, here as at
    /// a slot row: the notation rides on the declaration and persists,
    /// so the picker emits `SessionOp::SetVariableUnit` and the change
    /// enters the history like any other.
    ///
    /// Nothing is drawn for a dimension with no units to choose
    /// between (`Scalar`, `Count`): `widgets::pick_unit` offers
    /// `widgets::unit_choices`, which is empty for both.
    ///
    /// The combo itself is `widgets::pick_unit`, the creation forms'
    /// — the options, the width and the selected row are one
    /// question at every picker in the chrome. What this row adds is
    /// that a pick is an EDIT, and that re-picking the row already
    /// shown is not one.
    pub(crate) fn variable_unit_ui(&mut self, ui: &mut egui::Ui, row: &VariableRow) {
        let Some(written) = props::rendering_unit(row.dimension, row.unit, *self.notation) else {
            return;
        };
        if let Some(unit) = pick_unit(
            ui,
            "variable_unit",
            &row.var.full().to_string(),
            row.dimension,
            written,
        ) && unit != written
        {
            self.ops
                .push(SessionOp::SetVariableUnit { var: row.var, unit });
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
    /// Nothing is drawn at all for a dimension with no units to choose
    /// between (`Scalar`, `Count`; `widgets::unit_choices`).
    pub(crate) fn slot_unit_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, rows: &[SlotRow]) {
        let Some(first) = rows.first() else {
            return;
        };
        let options = unit_choices(first.dimension);
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
            .map(|(_, refusal)| crate::shown(refusal))
            .collect();
        let refusals = refusals.join("\n");
        let written: Vec<String> = writable.iter().map(|(row, _)| row.label.clone()).collect();
        let skipped: Vec<String> = refused.iter().map(|(row, _)| row.label.clone()).collect();
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

    /// What a slot has to SAY, under its row ([`Self::slot_readings_ui`]),
    /// then its offer and its naming field.
    pub(crate) fn slot_notes_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        self.slot_readings_ui(ui, node, row);
        self.offer_ui(ui, node, row.slot);
        self.name_field_ui(ui, node, row.slot);
    }

    /// The notes a slot says under its row ([`slot_notes`]), with the
    /// session's `range?` reading when one was taken for this field,
    /// and its stop while it is proving.
    fn slot_readings_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow) {
        let target = RangeTarget::Slot {
            node,
            slot: row.slot,
        };
        let (lines, proving) = self.range_lines(&target);
        let clicked = slot_notes(
            ui,
            &self.theme,
            row,
            &lines,
            self.session.doc(),
            *self.notation,
        );
        self.ops.extend(clicked.map(SessionOp::Select));
        if proving && stop_link(ui) {
            self.ops.push(SessionOp::StopRange);
        }
    }

    /// **The offer a typed value earns** (D10), under its row: each
    /// existing variable of equal value as a button whose click makes
    /// the slot read it ([`SessionOp::SetSlotVariable`]), and a button
    /// that declines, keeping the variable the typing minted
    /// ([`SessionOp::DeclineOffer`]). An unnamed variable's button
    /// opens the naming field instead, empty: a variable two slots
    /// share has a name (VR2), and the commit names it and shares it as
    /// one step. Nothing is drawn while nothing is offered
    /// (`DocSession::offered`).
    fn offer_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, slot: SlotId) {
        let offered = self.session.offered(node, slot);
        if offered.is_empty() {
            return;
        }
        ui.horizontal_wrapped(|ui| {
            crate::widgets::message_toned(ui, "same value as", &self.theme, Tone::Advisory);
            for offer in offered {
                let named = self.session.committed_doc().var_name(offer.var).is_some();
                let hover = if named { OFFER_HOVER } else { SHARE_HOVER };
                if !ui
                    .add(egui::Button::new(offer.label.as_str()).small())
                    .on_hover_text(hover)
                    .clicked()
                {
                    continue;
                }
                if named {
                    self.ops.push(SessionOp::SetSlotVariable {
                        node,
                        slot,
                        var: offer.var,
                        name: None,
                    });
                } else {
                    self.drafts.name_draft = Some(NameDraft {
                        node,
                        slot,
                        var: offer.var,
                        share: true,
                        text: String::new(),
                    });
                }
            }
            if ui
                .add(egui::Button::new("keep separate").small())
                .on_hover_text(DECLINE_HOVER)
                .clicked()
            {
                self.ops.push(SessionOp::DeclineOffer { node, slot });
            }
        });
    }

    /// The button that opens the naming field for the variable a slot
    /// reads — drawn only where that variable is unnamed, since a named
    /// one is renamed at its own row. The field opens empty: nothing
    /// proposes a name (Ev, PR 4247), and the document holds none until
    /// the person types one and commits it (VR2).
    fn name_button(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, row: &SlotRow, label: &str) {
        let Some(var) = self.unnamed(node, row) else {
            return;
        };
        if ui
            .add(egui::Button::new(label).small())
            .on_hover_text(NAME_HOVER)
            .clicked()
        {
            self.drafts.name_draft = Some(NameDraft {
                node,
                slot: row.slot,
                var,
                share: false,
                text: String::new(),
            });
        }
    }

    /// The unnamed variable `row`'s slot reads, which its naming
    /// button names — `None` where it reads a named one, or none.
    fn unnamed(&self, node: RecipeNodeId, row: &SlotRow) -> Option<VarId> {
        let doc = self.session.committed_doc();
        let var = doc.slot(node, row.slot)?;
        (doc.var_name(var).is_none() && doc.var(var).is_some()).then_some(var)
    }

    /// **The naming field**, under the row whose button opened it: the
    /// person's text, committed as one
    /// [`SessionOp::RenameVar`] of the variable the field was opened
    /// for — or, opened from an offer, as the
    /// [`SessionOp::SetSlotVariable`] that names it and makes the slot
    /// read it — on its button or Enter, abandoned on `cancel`. A text
    /// that is no name disables the commit and says why on hover, in
    /// the name door's words.
    ///
    /// The field stands while that variable is unnamed and the slot
    /// reads it, or it is still on offer there ([`NameDraft`]): a
    /// commit the door takes names it, and the field closes on the next
    /// frame; a commit the door refuses (a name taken) leaves it
    /// unnamed, so the field and its text stay for the person to amend.
    fn name_field_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId, slot: SlotId) {
        let Some(draft) = &self.drafts.name_draft else {
            return;
        };
        if (draft.node, draft.slot) != (node, slot) {
            return;
        }
        let (var, share) = (draft.var, draft.share);
        let doc = self.session.committed_doc();
        let opened_for = if share {
            self.session
                .offered(node, slot)
                .iter()
                .any(|offer| offer.var == var)
        } else {
            doc.slot(node, slot) == Some(var)
        };
        if !opened_for || doc.var_name(var).is_some() {
            self.drafts.name_draft = None;
            return;
        }
        let mut commit = None;
        let mut cancel = false;
        ui.horizontal(|ui| {
            let Some(draft) = self.drafts.name_draft.as_mut() else {
                return;
            };
            let field = ui.add(egui::TextEdit::singleline(&mut draft.text).desired_width(120.0));
            let entered = field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            let name = VarName::new(draft.text.trim());
            let button = ui.add_enabled(name.is_ok(), egui::Button::new("Name").small());
            let button = match &name {
                Ok(_) => button,
                Err(fault) => button.on_disabled_hover_text(fault.to_string()),
            };
            if entered || button.clicked() {
                commit = name.ok();
            }
            cancel = ui.add(egui::Button::new("cancel").small()).clicked();
        });
        if let Some(name) = commit {
            self.ops.push(if share {
                SessionOp::SetSlotVariable {
                    node,
                    slot,
                    var,
                    name: Some(name),
                }
            } else {
                SessionOp::RenameVar {
                    var,
                    name: Some(name),
                }
            });
        } else if cancel {
            self.drafts.name_draft = None;
        }
    }

    /// Whether a `range?` run for `target` is proving: its button then
    /// starts it again, dropping what it has proven.
    fn proving(&self, target: &RangeTarget) -> bool {
        self.session
            .range()
            .is_some_and(|reading| reading.target == *target && reading.proving())
    }

    /// **The session's `range?` reading for `target`**: its lines, and
    /// whether the run is still proving. No lines while no run was taken
    /// for that field, or the document changed since.
    fn range_lines(&self, target: &RangeTarget) -> (Vec<Sentence>, bool) {
        self.session
            .range()
            .filter(|reading| reading.target == *target)
            .map_or((Vec::new(), false), |reading| {
                (reading.said_lines(self.session.doc()), reading.proving())
            })
    }

    /// **The button that starts one slot's `range?` run.**
    ///
    /// Greyed ahead of the click by the kernel's admission door, which
    /// runs no geometry ([`crate::session::DocSession::range_refusal`]):
    /// the disabled control says the refused operation's own words. The
    /// reading lands in [`Self::slot_notes_ui`], in the slot's unit.
    pub(crate) fn range_button(
        &mut self,
        ui: &mut egui::Ui,
        node: RecipeNodeId,
        row: &SlotRow,
        label: &str,
    ) {
        let target = RangeTarget::Slot {
            node,
            slot: row.slot,
        };
        let refused = self.session.range_refusal(&target);
        let restarts = self.proving(&target);
        // A scalar row's button says it restarts; a vector's tagged one
        // keeps its axis tag and says so on hover.
        let label = if restarts && label == "range?" {
            "restart"
        } else {
            label
        };
        if range_control(ui, label, refused.as_ref(), restarts) {
            self.ops.push(SessionOp::ProveRange { target });
        }
    }

    /// The `range?` reading and button for a DOCUMENT VARIABLE — the one
    /// field that is not a slot: the reading on its own lines, its stop
    /// while it proves, and the button under them.
    pub(crate) fn variable_range_ui(&mut self, ui: &mut egui::Ui, row: &VariableRow) {
        let target = RangeTarget::Variable { var: row.var };
        let (lines, proving) = self.range_lines(&target);
        let refused = self.session.range_refusal(&target);
        let asked = range_notes(
            ui,
            &self.theme,
            &lines,
            self.session.doc(),
            proving,
            refused.as_ref(),
        );
        self.ops.extend(asked.followed.map(SessionOp::Select));
        if asked.stop {
            self.ops.push(SessionOp::StopRange);
        }
        if asked.range {
            self.ops.push(SessionOp::ProveRange { target });
        }
    }
}

/// Which tagged door a line of them offers ([`ViewerBehavior::tagged_doors`]).
#[derive(Clone, Copy, Debug)]
enum SlotDoor {
    /// `range?`.
    Range,
    /// The naming button.
    Name,
}

/// **A selected node's heading**: as the landed document holds it
/// ([`crate::tree::node_label`]) — the document the picture and the
/// face header beside it speak — and, where that document does not hold
/// it (deleted, or not landed yet), as the last document that held it
/// spoke it (`DocSession::selection_said`).
fn selected_node_heading(
    landed: Option<&Doc<ProfileProgram>>,
    files: &crate::parts::PartFiles,
    said: &HeldNodes,
    node: RecipeNodeId,
) -> Sentence {
    match landed.filter(|doc| doc.node(node).is_some()) {
        Some(doc) => crate::relations::Related::node(doc, node, files)
            .said()
            .clone(),
        None => Sentence::from(crate::tree::spelled(&said.spoken(node))),
    }
}

/// **Which way a relation row reads**: what the node reads, or what
/// reads the node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    /// `profile: Profile 1`.
    Reads,
    /// `profile of: Revolve 3`.
    ReadBy,
}

impl Direction {
    /// The heading over the rows.
    pub(crate) fn heading(self) -> &'static str {
        match self {
            Self::Reads => "reads",
            Self::ReadBy => "read by",
        }
    }

    /// A row's seat, as its line opens.
    pub(crate) fn seat(self, seat: crate::relations::Seat) -> String {
        match self {
            Self::Reads => format!("{}:", seat.label()),
            Self::ReadBy => format!("{} of:", seat.label()),
        }
    }
}

/// **A node's relation rows, drawn** ([`crate::relations::Relations`]):
/// under the direction's heading, each row its seat, then its entries,
/// then `through` and the named variable read, where there is one. Each
/// entry is a link that selects what it names, or words where it names
/// nothing this document holds. Nothing at all for no rows.
pub(crate) fn relation_rows(
    ui: &mut egui::Ui,
    on_screen: &Doc<ProfileProgram>,
    direction: Direction,
    rows: &[crate::relations::RelationRow],
    ops: &mut Vec<SessionOp>,
) {
    if rows.is_empty() {
        return;
    }
    ui.weak(direction.heading());
    for row in rows {
        ui.horizontal_wrapped(|ui| {
            ui.label(direction.seat(row.seat));
            for entry in &row.entries {
                related_entry(ui, on_screen, entry, ops);
            }
            if let Some(through) = &row.through {
                ui.label(THROUGH);
                related_entry(ui, on_screen, through, ops);
            }
        });
    }
}

/// The word between what a read reaches and the named variable it
/// reads it through; the placement editor's "copy of" row says it too.
pub(crate) const THROUGH: &str = "through";

/// One [`crate::relations::Related`], drawn ([`crate::widgets::sentence`]
/// against `on_screen`): its mention a link pushing its selection, the
/// rest words.
pub(crate) fn related_entry(
    ui: &mut egui::Ui,
    on_screen: &Doc<ProfileProgram>,
    entry: &crate::relations::Related,
    ops: &mut Vec<SessionOp>,
) {
    let said = crate::widgets::sentence(ui, entry.said(), on_screen, None);
    ops.extend(said.map(SessionOp::Select));
}

/// **A picked entity's header**: the pick as the chrome says one
/// ([`PickedName`]), and the node its Delete deletes — the feature the
/// words name ([`PickedName::feature`]), so the button deletes what the
/// label names. It is the node [`crate::session::FaceSelection::feature`]
/// answers too, so the tree highlights what the header deletes.
pub(crate) fn entity_header(
    picked: &PickedName<'_>,
    by: pncad::document::Speaker<'_>,
) -> (Sentence, RecipeNodeId) {
    (Said(picked, by).sentence(), picked.feature())
}

/// **What the selection's standing has to SAY, drawn**: the verdict on
/// a selection that no longer denotes — a deleted node, an undeclared
/// variable, a picked entity whose name did not resolve — in the
/// voice [`Standing::tone`] gives it, and the rebind count under a
/// failed name. Draws nothing for a selection that still denotes, or
/// for none.
///
/// A picked entity's noun is read off its own arm, so no caller can
/// hand this "face" for an edge.
///
/// The words are chosen per arm — for an undeclared variable,
/// [`Refusal::undeclared_wording`], the fact the refusal a drag on it
/// gets opens with; for a picked entity, the resolution machinery's
/// own payload — never a sentence composed here about somebody else's
/// refusal. How LOUD they are is not composed
/// per arm: it is read once, off the value.
///
/// A picked entity's resolution was asked of the landed run
/// (`DocSession::standing`), so its nodes are said from `landed`, the
/// document whose ids it is spelled in, and each name within that run's
/// tables; a node it no longer holds, as `said` keeps it, the last
/// document that held it (`DocSession::selection_said`); by their tags
/// when neither says it. The verdict is about the picked entity, so it
/// says "this face" and never its name again.
///
/// A free function over the `Ui` so a headless drive can reach it
/// (`crate::pane::headless`).
pub(crate) fn standing_verdict(
    ui: &mut egui::Ui,
    theme: &Theme,
    standing: &Standing,
    landed: Option<(&Doc<ProfileProgram>, &Evaluation<f64>)>,
    said: &HeldNodes,
) {
    let tone = standing.tone();
    let (noun, resolution) = match standing {
        Standing::Empty
        | Standing::Node { present: true, .. }
        | Standing::Variable { present: true, .. } => return,
        // One word, beside the node number the caller drew.
        Standing::Node { present: false, .. } => {
            ui.label(toned("deleted", theme, tone));
            return;
        }
        Standing::Variable {
            var,
            present: false,
        } => {
            crate::widgets::message_toned(ui, Refusal::undeclared_wording(var), theme, tone);
            return;
        }
        Standing::Face { resolution, .. } => ("face", resolution.as_deref()),
        Standing::Edge { resolution, .. } => ("edge", resolution.as_deref()),
        Standing::Vertex { resolution, .. } => ("vertex", resolution.as_deref()),
    };
    let by = crate::frame::landed_speaker(landed).or_held(said);
    let said = match resolution {
        None => Some("no evaluation yet to resolve this against".to_owned()),
        Some(Resolution::Resolved(_)) => None,
        Some(Resolution::Failed(failure)) => Some(
            Said(
                &AboutReference(&failure.error, format_args!("this {noun}")),
                by,
            )
            .sentence()
            .to_string(),
        ),
        Some(Resolution::Indeterminate(cause)) => Some(indeterminate_wording(noun, cause, by)),
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
fn exists_notice(ui: &mut egui::Ui, theme: &Theme, name: &VarName, dimension: Dimension) -> bool {
    // The already-exists sentence, and the edit door it offers instead
    // — refuse-then-offer, ahead of the click.
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
/// A slot with no one value — it did not evaluate, or it has one per
/// member — is still drawn, over a zero it does not show. A literal that
/// evaluated shows its number ([`props::field_text`]).
pub(crate) fn slot_showing(row: &SlotRow, draft: Option<&str>, notation: Notation) -> FieldShowing {
    let writing = FieldWriting::of(row.dimension, row.unit, notation);
    let number = props::slot_number(row, notation);
    let (text, source) = match draft {
        Some(draft) => (Some(draft.to_owned()), None),
        None => (props::field_text(row, notation), props::field_source(row)),
    };
    FieldShowing {
        name: None,
        writing,
        dimension: row.dimension,
        number,
        text,
        source,
    }
}

/// **A free variable's value field**: its number, in the unit it was
/// DECLARED in, with no text over it and no source to seed its edit
/// with. A defined one's is [`defined_showing`].
pub(crate) fn variable_showing(row: &VariableRow, notation: Notation) -> FieldShowing {
    let writing = FieldWriting::of(row.dimension, row.unit, notation);
    FieldShowing {
        name: None,
        writing,
        dimension: row.dimension,
        number: props::shown_value(writing.unit, row.value.as_f64()),
        text: None,
        source: None,
    }
}

/// **A free variable's line in the variables list**: its name and its
/// value, `t = 20 mm`. The value is said in the unit its own field
/// shows it in ([`variable_showing`]): the notation it was declared in,
/// or the working notation where it remembers none
/// ([`props::rendering_unit`]), so the list and the row say one value
/// in one unit. The digits are [`props::shown_text`]'s bounded readout,
/// where the field shows the number it edits.
pub(crate) fn variable_line(row: &VariableRow, notation: Notation) -> String {
    format!(
        "{} = {}",
        crate::tree::spelled_var(&row.label),
        props::shown_text(
            props::rendering_unit(row.dimension, row.unit, notation),
            row.value.as_f64()
        )
    )
}

/// **A defined variable's line in the variables list**: its name, its
/// definition and what that reads, `w = 2 * t = 40 mm` — the reading
/// as a driven slot's field says it ([`props::reading_text`]), with why
/// it reads nothing said under the line ([`reading_faults`]).
pub(crate) fn defined_line(row: &props::DefinedRow, notation: Notation) -> String {
    format!(
        "{} = {} {} {}",
        crate::tree::spelled_var(&row.label),
        row.formula,
        props::DRIVEN,
        props::reading_text(&row.value, row.dimension, notation)
    )
}

/// **A defined variable's value field**: its formula over the number
/// it evaluates to, and the formula again as the text an edit starts
/// from, as a driven slot's field shows its expression. A definition
/// with no one value — it refuses, or it has one per member — holds a
/// zero under its text, which no edit writes: a drag is refused by the
/// value door.
pub(crate) fn defined_showing(row: &props::DefinedRow, notation: Notation) -> FieldShowing {
    let writing = FieldWriting::of(row.dimension, None, notation);
    FieldShowing {
        name: None,
        writing,
        dimension: row.dimension,
        number: props::shown_value(
            writing.unit,
            row.value
                .as_ref()
                .ok()
                .and_then(props::Reading::one)
                .map_or(0.0, SlotValue::as_f64),
        ),
        text: Some(row.formula.clone()),
        source: Some(row.formula.clone()),
    }
}

/// **The two doors a variable's field has**: a bare number is a
/// value in the field's notation and nothing else moves; anything
/// else is text for `SessionOp::SetVariableText`, which reads a number
/// and its notation through the one parser and refuses what is
/// neither.
pub(crate) fn variable_doors(
    var: VarId,
) -> FieldVocabulary<impl Fn(SlotValue) -> SessionOp, impl Fn(String) -> SessionOp> {
    FieldVocabulary {
        number: move |value| SessionOp::SetVariable { var, value },
        text: move |text| SessionOp::SetVariableText { var, text },
    }
}

/// **The two doors a slot's field has**: a bare number through
/// `SessionOp::SetSlot`, which replaces an expression as it replaces a
/// literal, anything else through `SessionOp::SetSlotExpression`.
pub(crate) fn slot_doors(
    node: RecipeNodeId,
    slot: SlotId,
) -> FieldVocabulary<impl Fn(SlotValue) -> SessionOp, impl Fn(String) -> SessionOp> {
    FieldVocabulary {
        number: move |value| SessionOp::SetSlot { node, slot, value },
        text: move |text| SessionOp::SetSlotExpression { node, slot, text },
    }
}

/// **What a reading has to say under its row**, one line each, every
/// line naming `subject` — the slot or defined variable it is of:
///
/// * why it reads nothing, where it does not evaluate;
/// * its value at each member, where it has one per member
///   ([`props::MemberValues::spelled`]);
/// * each refusal among its members, with the members it refused at
///   ([`props::MemberValues::refusals`]).
///
/// Nothing for a reading of one value: its field shows it. The one home
/// of these lines for a slot row ([`slot_notes`]) and a defined
/// variable's.
pub(crate) fn reading_notes(
    subject: &str,
    value: &Result<props::Reading, props::SlotFault>,
    dimension: Dimension,
    notation: Notation,
) -> Vec<String> {
    let values = match value {
        Ok(props::Reading::PerMember(members)) => Some(format!(
            "{subject} at each member: {}",
            members.spelled(dimension, notation)
        )),
        Ok(props::Reading::One(_)) | Err(_) => None,
    };
    values
        .into_iter()
        .chain(reading_faults(subject, value))
        .collect()
}

/// **Why a reading reads nothing, wholly or at some members**, one line
/// each and every line naming `subject`: the fault where it does not
/// evaluate, each refusal among its members with the members it
/// refused at. Empty for a reading that refused nowhere. The part of
/// [`reading_notes`] a line that already says the reading's value
/// ([`defined_line`]) has left to say.
pub(crate) fn reading_faults(
    subject: &str,
    value: &Result<props::Reading, props::SlotFault>,
) -> Vec<String> {
    match value {
        Ok(props::Reading::One(_)) => Vec::new(),
        Ok(props::Reading::PerMember(members)) => members
            .refusals()
            .into_iter()
            .map(|line| format!("{subject} {line}"))
            .collect(),
        Err(error) => vec![format!("{subject}: {error}")],
    }
}

/// **What a slot has to SAY, under its row**:
///
/// * why it reads nothing, where it does not evaluate;
/// * its value at each member, where it has one per member, and every
///   member that refused with its refusal;
/// * a driven slot's expression, `label = source`, which its field
///   does not show ([`slot_showing`]), and the expression-driven
///   refusal's affordance with its edit doors;
/// * the range `reading`, when one has been taken for this field.
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
/// status line when they try. Its wording comes from
/// its one home, the same string the status line shows when the edit
/// is actually attempted. Its doors share a WRAPPING row, one per
/// variable the expression reads, so a door too long for what is
/// left of the line starts the next one.
///
/// Answers the variable whose door was clicked.
fn slot_notes(
    ui: &mut egui::Ui,
    theme: &Theme,
    row: &SlotRow,
    reading: &[Sentence],
    on_screen: &Doc<ProfileProgram>,
    notation: Notation,
) -> Option<Selection> {
    for line in reading_notes(&row.label, &row.value, row.dimension, notation) {
        crate::widgets::message_toned(ui, line, theme, Tone::Advisory);
    }
    let mut clicked = None;
    if let SlotDriver::Expression { variables } = &row.driver {
        if let Some(source) = &row.source {
            crate::widgets::message_toned(
                ui,
                format!("{} {} {source}", row.label, props::DRIVEN),
                theme,
                Tone::Advisory,
            );
        }
        crate::widgets::message_toned(
            ui,
            format!(
                "{}: {}",
                row.label,
                Refusal::affordance(
                    variables,
                    row.slot,
                    row.value.as_ref().ok().and_then(props::Reading::one),
                    notation,
                )
            ),
            theme,
            Tone::Advisory,
        );
        if !variables.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for var in variables {
                    if crate::widgets::message_link(
                        ui,
                        format!("edit {var}", var = crate::tree::spelled_var(var)),
                    )
                    .clicked()
                    {
                        clicked = Some(Selection::Variable(var.id()));
                    }
                }
            });
        }
    }
    for (i, line) in reading.iter().enumerate() {
        let line = if i == 0 {
            pncad::document::sentence!(format_args!("{}: ", row.label), line)
        } else {
            line.clone()
        };
        let followed =
            crate::widgets::sentence(ui, &line, on_screen, Some((theme, Tone::Advisory)));
        clicked = clicked.or(followed);
    }
    clicked
}

/// **The free-move probe's row**: the three translation components of
/// `current`, an eligible instance's display frame, written in the
/// working `notation`'s length unit, and the gesture that drags or
/// types them onto `ops`.
///
/// A LENGTH field written in the working notation — so the conversion
/// and the drag tick are the panel's own ([`FieldWriting`]) and the
/// probe reads in the unit every other value nobody wrote reads in.
/// Three components of one frame, one writing.
fn free_move_probe(
    ui: &mut egui::Ui,
    node: RecipeNodeId,
    current: [f64; 3],
    notation: Notation,
    ops: &mut Vec<SessionOp>,
) {
    let field = FieldWriting::of(Dimension::Length, None, notation);
    let unit = notation.length.def();
    ui.label(format!(
        "free-move probe ({}, display only):",
        unit.symbol()
    ));
    // **Asked, not formed**, as the panel's value fields ask
    // ([`crate::props::shown_value`]): a translation dragged or typed in
    // a coarse notation can have no value in a finer one the notation
    // was switched to.
    let over = match current.map(|canonical| crate::props::shown_value(field.unit, canonical)) {
        [Ok(x), Ok(y), Ok(z)] => [x, y, z],
        [Err(unit), ..] | [_, Err(unit), _] | [.., Err(unit)] => {
            crate::widgets::message(ui, crate::props::no_reading(unit));
            return;
        }
    };
    let mut written = over.map(crate::props::FieldNumber::shown);
    // The G1 gesture triple over DISPLAY state, through the one
    // widget→gesture mapping (`drag_ops`) so the typed-input arm exists
    // here too: typing a value performs a one-shot begin/preview/commit,
    // exactly one committed display value. The instance has ONE probe
    // and all three components drive it, so the row is one gesture and
    // `vec3_row_ops` maps it once — its docs carry what a triple per box
    // costs. Each preview composes the FULL frame from all three, so
    // dragging x does not zero y and z. The chrome offers the
    // translation components; the op vocabulary takes any rigid frame.
    let frame_of = |written: [f64; 3]| Frame::translation(written.map(|v| field.authored(v)));
    let ProbeOps { gesture, typed } = free_move_gesture(node, frame_of);
    ui.horizontal(|ui| {
        vec3_row_ops(ui, field, (&mut written, over), gesture, typed, ops);
    });
}

/// What a `range?` line's controls were asked this frame.
#[derive(Clone, Debug, Default, PartialEq)]
struct RangeAsked {
    /// The stop, under a run that is proving.
    stop: bool,
    /// The `range?` button.
    range: bool,
    /// What a mention clicked in the reading selects.
    followed: Option<Selection>,
}

/// **A document variable's `range?` reading and the button that starts
/// one** — each line on its own, the stop under them while the run
/// proves, and the button last, which is the order a slot's reading
/// ([`slot_notes`]) and its range row are drawn in.
///
/// The reading is sentences whose numbers are the search's, so none is
/// drawn beside a control: there it would be laid out from the
/// control's edge rather than the pane's.
fn range_notes(
    ui: &mut egui::Ui,
    theme: &Theme,
    lines: &[Sentence],
    on_screen: &Doc<ProfileProgram>,
    proving: bool,
    refused: Option<&Refusal>,
) -> RangeAsked {
    let mut followed = None;
    for line in lines {
        let clicked = crate::widgets::sentence(ui, line, on_screen, Some((theme, Tone::Advisory)));
        followed = followed.or(clicked);
    }
    RangeAsked {
        followed,
        stop: proving && stop_link(ui),
        range: range_control(
            ui,
            if proving { "restart" } else { "range?" },
            refused,
            proving,
        ),
    }
}

/// **The `range?` button**, greyed with `refused`'s own words where the
/// admission door refuses the run. Answers whether it was clicked.
fn range_control(
    ui: &mut egui::Ui,
    label: &str,
    refused: Option<&Refusal>,
    restarts: bool,
) -> bool {
    let button = ui.add_enabled(refused.is_none(), egui::Button::new(label).small());
    match refused {
        None => button.on_hover_text(if restarts { RESTART_HOVER } else { RANGE_HOVER }),
        // The refusal renders itself; nothing here composes words.
        Some(refusal) => button.on_disabled_hover_text(crate::shown(refusal)),
    }
    .clicked()
}

/// **The stop under a proving `range?` run**: it keeps what the run has
/// proven. Answers whether it was clicked.
fn stop_link(ui: &mut egui::Ui) -> bool {
    crate::widgets::message_link(ui, "stop")
        .on_hover_text(STOP_HOVER)
        .clicked()
}

/// **The body's toggle**: answers whether it was changed.
fn body_toggle(ui: &mut egui::Ui, on: &mut bool) -> bool {
    ui.checkbox(on, "shown in viewport").changed()
}

/// What an offered variable's button says on hover.
const OFFER_HOVER: &str = "read this variable here instead of the value typed: the two slots then \
                           move together";

/// What an unnamed offered variable's button says on hover.
const SHARE_HOVER: &str = "name this variable to read it here instead of the value typed: a \
                           variable two slots share has a name";

/// What the decline button says on hover.
const DECLINE_HOVER: &str = "keep the value typed as its own variable";

/// What the naming button says on hover.
const NAME_HOVER: &str = "name the variable this slot reads, so a formula can read it too";

/// What a range button says on hover, for a slot's and a variable's
/// alike.
const RANGE_HOVER: &str = "prove how far this can move and stay the same build — it keeps \
                           proving until you stop it or it can go no further, which on some \
                           fields is never";

/// What the button says on hover while its run is proving.
const RESTART_HOVER: &str = "start proving again from nothing, dropping what this run has proven";

/// What the stop under a proving run says on hover.
const STOP_HOVER: &str = "stop proving, and keep what is proven so far";

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

    use pncad::document::{Dimension, SlotId, SpokenVar, VarId, VarName};

    use super::{exists_notice, range_notes, reading_notes, slot_notes, slot_showing};
    use crate::pane::headless::{assert_inside, assert_own_lines, assert_under, drawn_in, find};
    use crate::props::{
        MemberValues, Notation, Reading, SlotDriver, SlotFault, SlotRow, SlotValue,
    };
    use crate::session::Refusal;
    use crate::theme::Theme;

    /// A narrow pane, and wider than `crate::widgets::message_floor`,
    /// so these rows read the region and not the floor.
    const REGION: f32 = 260.0;

    /// A named variable, its id derived from the name so two names
    /// are two variables.
    fn variable(name: &'static str) -> SpokenVar {
        let id = name
            .bytes()
            .fold(0_u64, |h, b| h.wrapping_mul(31).wrapping_add(u64::from(b)));
        SpokenVar::new(VarId::new(0, id), Some(VarName::from_static(name)))
    }

    /// An extrude distance row with `driver` and `value`.
    fn distance_row(driver: SlotDriver, value: Result<SlotValue, SlotFault>) -> SlotRow {
        SlotRow {
            slot: SlotId::Distance,
            label: SlotId::Distance.label(),
            dimension: Dimension::Length,
            structural: false,
            driver,
            value: value.map(crate::props::Reading::One),
            unit: None,
            source: None,
        }
    }

    /// **What the pane draws under a row read per member**: its value
    /// at each member, and each refusal once with the members it
    /// refused at — read off what is painted, not off the strings the
    /// row hands it.
    #[test]
    fn a_per_member_rows_values_and_refusals_are_drawn_under_it() {
        let k = variable("k");
        let refused = pncad::document::EvalError::NonFiniteResult;
        let members = MemberValues::new(
            vec![k.clone()],
            (0..10)
                .map(|at| {
                    let value = if at == 3 || at == 7 {
                        Err(refused.clone())
                    } else {
                        Ok(SlotValue::Continuous(0.001 * at as f64))
                    };
                    (vec![at], value)
                })
                .collect(),
        );
        let spelled = members.spelled(Dimension::Length, Notation::DEFAULT);
        let row = SlotRow {
            value: Ok(Reading::PerMember(members)),
            ..distance_row(
                SlotDriver::Expression {
                    variables: vec![k.clone()],
                },
                Ok(SlotValue::Continuous(0.0)),
            )
        };
        let (region, painted) = drawn_in(REGION, |ui| {
            slot_notes(
                ui,
                &Theme::DEFAULT,
                &row,
                &[],
                &pncad::document::Doc::empty_derived(
                    "slot-notes",
                    pncad::geom_core::Tol::witness(),
                ),
                Notation::DEFAULT,
            );
        });
        let label = SlotId::Distance.label();
        let values = find(&painted, &format!("{label} at each member: {spelled}"));
        assert_own_lines(region, values);
        let refusal = find(
            &painted,
            &format!(
                "{label} refused at {k} = 3, 7: {refused}",
                k = crate::tree::spelled_var(&k)
            ),
        );
        assert_own_lines(region, refusal);
        assert_under(values, refusal);
    }

    /// **A reading that refuses says why**, a defined variable's as a
    /// slot's: the one home of the lines under a reading.
    #[test]
    fn a_refused_reading_says_its_fault() {
        let fault = SlotFault::Eval(pncad::document::EvalError::NonFiniteResult);
        assert_eq!(
            reading_notes(
                "step",
                &Err(fault.clone()),
                Dimension::Length,
                Notation::DEFAULT
            ),
            [format!("step: {fault}")]
        );
    }

    #[test]
    fn a_declared_names_notice_and_its_door_each_take_a_line_inside_the_pane() {
        let name = VarName::from_static("outer_enclosure_wall_thickness");
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
        let variables = vec![
            variable("outer_enclosure_wall_thickness"),
            variable("lid_clearance"),
            variable("gasket_compression_allowance"),
        ];
        let value = SlotValue::of(Dimension::Length, 0.004).expect("a finite length");
        let row = distance_row(
            SlotDriver::Expression {
                variables: variables.clone(),
            },
            Ok(value),
        );
        let (region, painted) = drawn_in(REGION, |ui| {
            slot_notes(
                ui,
                &Theme::DEFAULT,
                &row,
                &[],
                &pncad::document::Doc::empty_derived(
                    "slot-notes",
                    pncad::geom_core::Tol::witness(),
                ),
                Notation::DEFAULT,
            );
        });
        let affordance = find(
            &painted,
            &format!(
                "{}: {}",
                SlotId::Distance.label(),
                Refusal::affordance(&variables, SlotId::Distance, Some(value), Notation::DEFAULT)
            ),
        );
        assert_own_lines(region, affordance);
        for var in &variables {
            let door = find(
                &painted,
                &format!("edit {var}", var = crate::tree::spelled_var(var)),
            );
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
                    variables: vec![variable("outer_enclosure_wall_thickness")],
                },
                Ok(value),
            )
        };
        let (region, painted) = drawn_in(REGION, |ui| {
            slot_notes(
                ui,
                &Theme::DEFAULT,
                &row,
                &[],
                &pncad::document::Doc::empty_derived(
                    "slot-notes",
                    pncad::geom_core::Tol::witness(),
                ),
                Notation::DEFAULT,
            );
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
                    variables: vec![variable("thickness")],
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
            slot_notes(
                ui,
                &Theme::DEFAULT,
                &row,
                &[],
                &pncad::document::Doc::empty_derived(
                    "slot-notes",
                    pncad::geom_core::Tol::witness(),
                ),
                Notation::DEFAULT,
            );
        });
        let fault = find(
            &painted,
            &format!("{}: {}", SlotId::Distance.label(), SlotFault::NoExpression),
        );
        assert_own_lines(region, fault);
    }

    #[test]
    fn a_variables_range_reading_is_said_over_its_stop_and_button_inside_the_pane() {
        let lines = [
            "proven 0.0012345678901234567 m…12.345678901234567 m".to_owned(),
            "checked to 0 m…24.691357802469134 m".to_owned(),
        ];
        let (region, painted) = drawn_in(REGION, |ui| {
            range_notes(
                ui,
                &Theme::DEFAULT,
                &lines.clone().map(pncad::document::Sentence::from),
                &pncad::document::Doc::empty_derived(
                    "range-notes",
                    pncad::geom_core::Tol::witness(),
                ),
                true,
                None,
            );
        });
        let button = find(&painted, "restart");
        let stop = find(&painted, "stop");
        for line in &lines {
            let said = find(&painted, line);
            assert_own_lines(region, said);
            assert_under(said, stop);
        }
        assert_under(stop, button);
    }
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use std::cell::Cell;

    use super::{body_toggle, free_move_probe};
    use crate::pane::headless::{painted_after_clicking, painted_text};
    use crate::props::Notation;
    use crate::session::{Refusal, SessionOp};
    use eframe::egui;
    use pncad::document::RecipeNodeId;

    const NODE: RecipeNodeId = RecipeNodeId::new(0, test_utils::refusal::tagged(4));

    /// **The header's Delete deletes the node its label calls the
    /// feature, the node the tree highlights.** A name the attribution
    /// walk cannot classify (here, no path) is said as made by its own
    /// node; the header deletes that one, says the drawn node as where
    /// the pick was taken, and `FaceSelection::feature` — what the tree
    /// highlights — answers the same node. A classified cap likewise.
    #[test]
    fn the_entity_headers_delete_is_the_feature_its_label_names() {
        use crate::session::FaceSelection;
        use crate::tree::PickedName;
        use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
        let node = |n| RecipeNodeId::new(0, test_utils::refusal::tagged(n));
        let (named, drawn) = (node(3), node(5));
        let face = |path| FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: named,
                path,
            },
            node: drawn,
            body: 0,
        };
        let unclassified = face(Vec::new());
        assert_eq!(
            unclassified.feature(),
            named,
            "the tree's feature is the header's"
        );
        let by = pncad::document::Speaker::TAG;
        let (said, feature) = super::entity_header(&PickedName::face(&unclassified), by);
        assert_eq!(
            (said.to_string(), feature),
            (
                "the face of node 000000000003, on node 000000000005".to_owned(),
                named
            ),
        );
        let cap = face(vec![RoleSeg::Cap(CapEnd::End)]);
        let (said, feature) = super::entity_header(&PickedName::face(&cap), by);
        assert_eq!(
            (said.to_string(), feature),
            (
                "the end cap of node 000000000003, on node 000000000005".to_owned(),
                named
            ),
        );
        assert_eq!(cap.feature(), named, "the tree's feature is the header's");
    }

    /// **Admission greys `range?`**: a refused run's button takes no
    /// click, and an admitted one does — the same control, with and
    /// without the refusal the session's admission door answers.
    #[test]
    fn a_refused_range_button_takes_no_click() {
        for (refused, want) in [
            (
                Some(Refusal::Range(Box::new(
                    pncad::analysis::RangeRefusal::SeedNotABracket { lo: 1.0, hi: 2.0 },
                ))),
                false,
            ),
            (None, true),
        ] {
            let clicked = Cell::new(false);
            painted_after_clicking("range?", |ui| {
                if super::range_control(ui, "range?", refused.as_ref(), false) {
                    clicked.set(true);
                }
            });
            assert_eq!(clicked.get(), want, "refused: {refused:?}");
        }
    }

    /// **A click on the body's toggle flips it.**
    #[test]
    fn a_body_toggle_flips() {
        let changed = Cell::new(false);
        painted_after_clicking("shown in viewport", |ui| {
            let mut on = true;
            if body_toggle(ui, &mut on) {
                changed.set(true);
            }
        });
        assert!(changed.get(), "the click reached the checkbox");
    }

    /// The working notation every free-move row below is drawn in:
    /// millimetres, which is not the default, so a probe that kept a
    /// fixed unit reds whichever one it kept.
    const MILLIMETRES: Notation = Notation {
        length: pncad::quantity::MM,
        ..Notation::DEFAULT
    };

    /// **The free-move probe reads in the working notation**: its label
    /// names the notation's length unit, and its boxes show the
    /// translation in it.
    #[test]
    fn the_free_move_probe_is_labelled_and_shown_in_the_working_unit() {
        let painted = painted_text(|ui| {
            free_move_probe(ui, NODE, [0.25, 0.0, 0.0], MILLIMETRES, &mut Vec::new());
        });
        assert!(
            painted.contains("free-move probe (mm, display only):"),
            "the label names the working unit: {painted}"
        );
        assert!(
            painted
                .lines()
                .any(|line| line.trim().parse::<f64>() == Ok(250.0)),
            "and 0.25 m shows as 250 in it: {painted}"
        );
    }

    /// **A number typed into the probe commits in the working unit** —
    /// `5` in a millimetre box previews a frame 5 mm along x, which is
    /// `field.authored` and not the number as typed.
    #[test]
    fn a_number_typed_into_the_free_move_probe_commits_in_the_working_unit() {
        let ctx = egui::Context::default();
        let mut emitted: Vec<SessionOp> = Vec::new();
        let mut frame = |events: Vec<egui::Event>| {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            let mut output = ctx.run_ui(input, |ui| {
                free_move_probe(ui, NODE, [0.0; 3], MILLIMETRES, &mut emitted);
            });
            // The font atlas is built on the first pass and epaint
            // panics on a dropped delta nobody uploaded; nothing here
            // paints.
            output.textures_delta.clear();
        };
        let key = |key| egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        frame(Vec::new());
        // Tab lands on x, which takes the focus with its text selected,
        // so the typed number replaces it.
        frame(vec![key(egui::Key::Tab)]);
        frame(vec![egui::Event::Text("5".to_owned())]);
        frame(vec![key(egui::Key::Enter)]);
        let previewed: Vec<[f64; 3]> = emitted
            .iter()
            .filter_map(|op| match op {
                SessionOp::PreviewFreeMove { node, frame } if *node == NODE => {
                    Some(frame.translation)
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            previewed,
            [[5.0 * 1.0e-3, 0.0, 0.0]],
            "one typed value, previewed as millimetres: {emitted:?}"
        );
        assert!(
            emitted
                .iter()
                .any(|op| matches!(op, SessionOp::CommitFreeMove { node } if *node == NODE)),
            "and committed: {emitted:?}"
        );
    }

    /// **A translation the working unit cannot name has no box to show
    /// it in**: the row says which notation could not name it and draws
    /// no field, where a coarser notation still shows the same value.
    #[test]
    fn a_free_move_the_working_unit_cannot_name_draws_no_reading() {
        let unnameable = [1.0e306, 0.0, 0.0];
        let mut emitted = Vec::new();
        let painted = painted_text(|ui| {
            free_move_probe(ui, NODE, unnameable, MILLIMETRES, &mut emitted);
        });
        assert!(
            painted.contains("no mm reading"),
            "the row says the millimetre value does not exist: {painted}"
        );
        assert!(
            !painted.contains("inf"),
            "and spells no infinity: {painted}"
        );
        assert!(emitted.is_empty(), "with no field there is nothing to emit");
        let metres = painted_text(|ui| {
            free_move_probe(ui, NODE, unnameable, Notation::DEFAULT, &mut Vec::new());
        });
        assert!(
            !metres.contains("no m reading"),
            "metres still name it, so the refusal is the notation's: {metres}"
        );
    }
}

/// **The variables list says each variable's value**, in the unit its
/// own field shows it in.
#[cfg(test)]
mod variable_lines {
    use pncad::document::{Dimension, EvalError, SpokenVar, VarId, VarName};

    use super::{defined_line, reading_faults, reading_notes, variable_line};
    use crate::props::{
        DefinedRow, NO_VALUE, Notation, Reading, SlotFault, SlotValue, VariableRow,
    };

    fn spoken(name: &'static str) -> SpokenVar {
        SpokenVar::new(VarId::new(0, 7), Some(VarName::from_static(name)))
    }

    fn defined(value: Result<Reading, SlotFault>) -> DefinedRow {
        DefinedRow {
            var: VarId::new(0, 8),
            label: spoken("w"),
            dimension: Dimension::Length,
            formula: "2 * t".to_owned(),
            value,
        }
    }

    /// A length declared in millimetres reads in millimetres whatever
    /// the working notation; a count reads as the number it is.
    #[test]
    fn a_free_variable_reads_its_value_in_its_own_unit() {
        let row = VariableRow {
            var: VarId::new(0, 7),
            label: spoken("t"),
            dimension: Dimension::Length,
            value: SlotValue::Continuous(0.02),
            unit: Some(pncad::quantity::MM.def()),
        };
        assert_eq!(variable_line(&row, Notation::DEFAULT), "t = 20 mm");
        let count = VariableRow {
            dimension: Dimension::Count,
            value: SlotValue::Count(3),
            unit: None,
            ..row
        };
        assert_eq!(variable_line(&count, Notation::DEFAULT), "t = 3");
    }

    /// The working notation is the one a definition reads in, and a
    /// free variable remembering no unit reads in it too (review of
    /// #6242): under millimetres, `0.04 m` is `40 mm`, so a line that
    /// kept the default notation goes red.
    #[test]
    fn the_working_notation_is_what_an_unwritten_value_reads_in() {
        let millimetres = Notation {
            length: pncad::quantity::MM,
            ..Notation::DEFAULT
        };
        assert_eq!(
            defined_line(
                &defined(Ok(Reading::One(SlotValue::Continuous(0.04)))),
                millimetres
            ),
            "w = 2 * t = 40 mm"
        );
        let unremembered = VariableRow {
            var: VarId::new(0, 7),
            label: spoken("t"),
            dimension: Dimension::Length,
            value: SlotValue::Continuous(0.02),
            unit: None,
        };
        assert_eq!(variable_line(&unremembered, millimetres), "t = 20 mm");
    }

    /// A definition reads its value in the working notation, and one
    /// that reads nothing says the field's `?`.
    #[test]
    fn a_defined_variable_reads_its_definition_and_its_value() {
        assert_eq!(
            defined_line(
                &defined(Ok(Reading::One(SlotValue::Continuous(0.04)))),
                Notation::DEFAULT
            ),
            format!(
                "w = 2 * t = {}",
                crate::props::computed_text(Dimension::Length, 0.04, Notation::DEFAULT)
            )
        );
        assert_eq!(
            defined_line(
                &defined(Err(SlotFault::Eval(EvalError::NonFiniteResult))),
                Notation::DEFAULT
            ),
            format!("w = 2 * t = {NO_VALUE}")
        );
    }

    /// **A definition refused at some members says why, as its row
    /// does**: the list line shows the span, and the lines under it are
    /// the refusals the selected row's notes carry — every note but the
    /// values line, which the span already says.
    #[test]
    fn a_partly_refused_definition_says_its_refusals_as_its_row_does() {
        let members = crate::props::MemberValues::new(
            vec![spoken("k")],
            vec![
                (vec![0], Ok(SlotValue::Continuous(0.01))),
                (vec![1], Err(EvalError::NonFiniteResult)),
            ],
        );
        let value = Ok(Reading::PerMember(members));
        let faults = reading_faults("w", &value);
        assert!(!faults.is_empty(), "the refused member is said");
        let notes = reading_notes("w", &value, Dimension::Length, Notation::DEFAULT);
        assert_eq!(
            notes[1..],
            faults[..],
            "the row's refusal lines, and only those"
        );
        assert!(
            defined_line(&defined(value), Notation::DEFAULT).contains(NO_VALUE),
            "the span says a member refused"
        );
    }
}

/// **How loud the selection's verdict is drawn**, read off the paint
/// and held against fixed colours ([`crate::pane::headless::Voices`]),
/// through [`standing_verdict`] — the function the header calls — so
/// a literal tone put back at any of its arms, or a swapped mapping
/// anywhere between [`Standing::tone`] and the glyphs, turns a row red.
#[cfg(test)]
mod verdict_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used, clippy::panic)]

    use editor_core::RecipeEditRef;
    use pncad::document::{
        Doc, HeldNodes, Label, NodeStanding, ProfileProgram, RecipeNodeId, SpokenVar, VarId,
        VarName,
    };
    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
    use pncad::select::{Resolution, ResolutionFailure, ResolveError, ResolveIndeterminate};

    use super::standing_verdict;
    use crate::pane::headless::{Landed, Voices, find, find_opening, landed_voiced};
    use crate::session::{
        DocSession, EdgeSelection, FaceSelection, Selection, SessionOp, Standing,
    };
    use crate::theme::Theme;

    fn name(kind: EntityKind) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(1)),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        }
    }

    fn face(resolution: Option<Resolution>) -> Standing {
        Standing::Face {
            face: FaceSelection {
                name: name(EntityKind::Face),
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(2)),
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
                    node: RecipeNodeId::new(0, test_utils::refusal::tagged(1)),
                },
            },
            offers,
        })
    }

    /// What [`standing_verdict`] painted for `standing`, with nothing
    /// landed.
    fn drawn(standing: &Standing) -> (Vec<Landed>, Voices) {
        drawn_over(standing, None)
    }

    /// What [`standing_verdict`] painted for `standing` over `landed`,
    /// keeping nothing of the selection's nodes.
    fn drawn_over(
        standing: &Standing,
        landed: Option<(&Doc<ProfileProgram>, &pncad::document::Evaluation<f64>)>,
    ) -> (Vec<Landed>, Voices) {
        drawn_keeping(standing, landed, &HeldNodes::default())
    }

    /// What [`standing_verdict`] painted for `standing` over `landed`,
    /// with `said` kept of the selection's nodes.
    fn drawn_keeping(
        standing: &Standing,
        landed: Option<(&Doc<ProfileProgram>, &pncad::document::Evaluation<f64>)>,
        said: &HeldNodes,
    ) -> (Vec<Landed>, Voices) {
        landed_voiced(&Theme::DEFAULT, |ui, theme| {
            standing_verdict(ui, theme, standing, landed, said)
        })
    }

    /// **A picked entity's verdict says its nodes as the landed
    /// document holds them**: the failed arm's name and the
    /// indeterminate arm's standing each say the block by its label
    /// over the document they were asked of, and by its tag with none;
    /// a minting node that document no longer holds, by its tag when
    /// the selection kept nothing.
    #[test]
    fn a_pick_verdict_says_its_nodes_from_the_landed_document() {
        let tol = pncad::tolerance::witness();
        let (mut doc, block, _) = crate::test_support::boss_on_block("verdict-speaks", tol);
        crate::test_support::edit_into(
            &mut doc,
            pncad::document::DocEdit::SetLabel {
                node: block,
                label: Some(pncad::document::Label::new("base block").expect("a label")),
            },
            tol,
        );
        let landed = pncad::document::evaluate(
            &doc,
            None,
            &pncad::document::CancelToken::new(),
            &pncad::document::EvalOptions::default(),
            tol,
        );
        let by_tag = format!("node {}", test_utils::refusal::tag(block.0.digest()));
        let name = StableName {
            kind: EntityKind::Face,
            node: block,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        };
        let face = |resolution: Resolution| Standing::Face {
            face: FaceSelection {
                name: name.clone(),
                node: block,
                body: 0,
            },
            resolution: Some(Box::new(resolution)),
        };
        let gone = face(Resolution::Failed(ResolutionFailure {
            error: ResolveError::Vanished {
                name: name.clone(),
                diagnosis: editor_core::Diagnosis::BorderDelta {
                    node: block,
                    gone: Vec::new(),
                    new: Vec::new(),
                },
                last_good: None,
            },
            offers: Vec::new(),
        }));
        let waiting = face(Resolution::Indeterminate(ResolveIndeterminate {
            standing: NodeStanding::Failed { node: block },
        }));
        for (arm, standing, opening) in [
            (
                "failed",
                &gone,
                "this face no longer resolves in this evaluation: ",
            ),
            (
                "indeterminate",
                &waiting,
                "this face cannot be resolved right now: ",
            ),
        ] {
            let (over_doc, _) = drawn_over(standing, Some((&doc, &landed)));
            let said = &find_opening(&over_doc, opening).text;
            assert!(
                said.contains("base block") && !said.contains(&by_tag),
                "the {arm} verdict says the block by its label over the landed document: {said}"
            );
            let (untied, _) = drawn(standing);
            let said = &find_opening(&untied, opening).text;
            assert!(
                said.contains(&by_tag) && !said.contains("base block"),
                "the {arm} verdict says the block by its tag with nothing landed: {said}"
            );
        }

        // A node the landed document no longer holds has no label to
        // say: it is said by its tag over that document too.
        let deleted = RecipeNodeId::new(0, test_utils::refusal::tagged(1));
        assert!(doc.node(deleted).is_none(), "the fixture's id is absent");
        let stranded = StableName {
            kind: EntityKind::Face,
            node: deleted,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        };
        let (over_doc, _) = drawn_over(
            &face(Resolution::Failed(ResolutionFailure {
                error: ResolveError::NodeGone {
                    name: stranded,
                    edit: RecipeEditRef::NodeDeleted { node: deleted },
                },
                offers: Vec::new(),
            })),
            Some((&doc, &landed)),
        );
        let said = &find_opening(&over_doc, "this face is stranded: ").text;
        assert!(
            !said.contains("end cap"),
            "the verdict is about this face, and does not say its name again: {said}"
        );
        let deleted_by_tag = format!("node {}", test_utils::refusal::tag(deleted.0.digest()));
        assert!(
            said.contains(&deleted_by_tag),
            "a deleted minting node is said by its tag over the landed document: {said}"
        );
    }

    /// **A selection whose node is deleted says it as it was picked**:
    /// the session keeps the selection's nodes spoken when it is made, so
    /// once the landed document no longer holds the face's minting node
    /// the verdict says the label it had, and a selected node deleted
    /// since heads the pane with it. Kept nothing, both say the tag.
    #[test]
    fn a_selection_whose_node_is_deleted_says_the_label_it_had_when_picked() {
        let tol = pncad::tolerance::witness();
        let (doc, block, _) = crate::test_support::boss_on_block("verdict-keeps", tol);
        let picked_face = Selection::Face(FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: block,
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node: block,
            body: 0,
        });
        let labelled = format!(
            "\"base block\" ({})",
            test_utils::refusal::tag(block.0.digest())
        );
        let by_tag = format!("node {}", test_utils::refusal::tag(block.0.digest()));
        for selection in [picked_face, Selection::Node(block)] {
            let mut session = DocSession::inline(doc.clone(), tol);
            let named = session.perform(SessionOp::SetLabel {
                node: block,
                label: Some(Label::new("base block").expect("a label")),
            });
            assert!(named.refusal.is_none(), "{:?}", named.refusal);
            session.pump();
            session.perform(SessionOp::Select(selection.clone()));
            let deleted = session.perform(SessionOp::DeleteNode { node: block });
            assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
            session.pump();
            let (landed, evaluation) = session.landed_pair().expect("the delete landed");
            assert!(
                landed.node(block).is_none(),
                "the landed document lost the block"
            );
            let said = session.selection_said();
            match (&selection, session.standing()) {
                (Selection::Face(_), standing @ Standing::Face { .. }) => {
                    let (kept, _) = drawn_keeping(&standing, Some((landed, evaluation)), said);
                    let text = &find_opening(&kept, "this face is stranded: ").text;
                    assert!(
                        text.contains(&labelled) && !text.contains(&by_tag),
                        "the verdict says the minting node as it was picked: {text}"
                    );
                    let (bare, _) = drawn_over(&standing, Some((landed, evaluation)));
                    let text = &find_opening(&bare, "this face is stranded: ").text;
                    assert!(
                        text.contains(&by_tag),
                        "kept nothing, it is said by its tag: {text}"
                    );
                }
                (
                    Selection::Node(_),
                    Standing::Node {
                        node,
                        present: false,
                    },
                ) => {
                    let files = crate::parts::PartFiles::default();
                    assert_eq!(
                        super::selected_node_heading(Some(landed), &files, said, node),
                        labelled,
                        "a deleted node heads the pane by the label it had"
                    );
                    assert_eq!(
                        super::selected_node_heading(
                            Some(landed),
                            &files,
                            &HeldNodes::default(),
                            node,
                        ),
                        by_tag,
                        "kept nothing, it is said by its tag"
                    );
                }
                (_, other) => panic!("{selection:?} stands deleted as {other:?}"),
            }
        }
    }

    /// **A pick off the landed run is kept from it**: the viewport and
    /// the Checks window pick from the landed run, which can still hold
    /// a node the shown document has just deleted, so the selection
    /// keeps the node as the landed document says it.
    #[test]
    fn a_selection_keeps_what_the_document_it_was_picked_in_says() {
        let tol = pncad::tolerance::witness();
        let (doc, block, _) = crate::test_support::boss_on_block("verdict-picked-in", tol);
        let t = test_utils::refusal::tag(block.0.digest());
        let face = Selection::Face(FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: block,
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node: block,
            body: 0,
        });

        let mut session = DocSession::inline(doc, tol);
        let named = session.perform(SessionOp::SetLabel {
            node: block,
            label: Some(Label::new("base block").expect("a label")),
        });
        assert!(named.refusal.is_none(), "{:?}", named.refusal);
        session.pump();
        let deleted = session.perform(SessionOp::DeleteNode { node: block });
        assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
        assert!(
            session.doc().node(block).is_none(),
            "the shown document lost the block before the pick"
        );
        session.perform(SessionOp::Select(face));
        assert_eq!(
            crate::tree::spelled(&session.selection_said().spoken(block)),
            format!("\"base block\" ({t})"),
            "the landed run still held the block, so the pick keeps its label"
        );
    }

    /// **A tree pick is kept from the shown document**: a node inserted
    /// since the last landing is held by the shown document alone, and
    /// the feature tree draws it there, so the selection keeps it as
    /// the shown document says it.
    #[test]
    fn a_tree_pick_of_a_node_not_yet_landed_keeps_its_label() {
        let tol = pncad::tolerance::witness();
        let (doc, _, _) = crate::test_support::boss_on_block("verdict-unlanded", tol);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let added = session.perform(SessionOp::AddProfile {
            loops: vec![crate::test_support::rectangle_loop([0.0, 0.0], 0.01, 0.01)],
        });
        assert!(added.refusal.is_none(), "{:?}", added.refusal);
        let profile = *session.doc().ids().last().expect("the added profile");
        let named = session.perform(SessionOp::SetLabel {
            node: profile,
            label: Some(Label::new("floor").expect("a label")),
        });
        assert!(named.refusal.is_none(), "{:?}", named.refusal);
        assert!(
            session
                .landed_pair()
                .is_some_and(|(landed, _)| landed.node(profile).is_none()),
            "the landed run does not hold the profile yet"
        );
        let labelled = crate::tree::spelled(&session.doc().spoken(profile));
        assert!(labelled.contains("\"floor\""), "{labelled}");
        session.perform(SessionOp::Select(Selection::Node(profile)));
        let deleted = session.perform(SessionOp::DeleteNode { node: profile });
        assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
        session.pump();
        let files = crate::parts::PartFiles::default();
        assert_eq!(
            super::selected_node_heading(
                session.landed_pair().map(|(doc, _)| doc),
                &files,
                session.selection_said(),
                profile,
            ),
            labelled,
            "the deleted profile heads the pane as the shown document said it"
        );
    }

    /// **The kept nodes follow the last document that held them**: a
    /// label written or changed after the pick is the one a deleted
    /// node is said by, not the one it had when it was picked.
    #[test]
    fn a_deleted_selection_says_the_last_label_its_node_had() {
        let tol = pncad::tolerance::witness();
        let (doc, block, _) = crate::test_support::boss_on_block("verdict-last-label", tol);
        let t = test_utils::refusal::tag(block.0.digest());
        let label = |name: &str| Some(Label::new(name).expect("a label"));
        for (what, at_pick, after) in [
            ("unlabelled, then labelled", None, "base"),
            ("labelled, then renamed", label("a"), "b"),
        ] {
            let mut session = DocSession::inline(doc.clone(), tol);
            let named = session.perform(SessionOp::SetLabel {
                node: block,
                label: at_pick,
            });
            assert!(named.refusal.is_none(), "{what}: {:?}", named.refusal);
            session.pump();
            session.perform(SessionOp::Select(Selection::Node(block)));
            let renamed = session.perform(SessionOp::SetLabel {
                node: block,
                label: label(after),
            });
            assert!(renamed.refusal.is_none(), "{what}: {:?}", renamed.refusal);
            let deleted = session.perform(SessionOp::DeleteNode { node: block });
            assert!(deleted.refusal.is_none(), "{what}: {:?}", deleted.refusal);
            session.pump();
            assert_eq!(
                super::selected_node_heading(
                    session.landed_pair().map(|(doc, _)| doc),
                    &crate::parts::PartFiles::default(),
                    session.selection_said(),
                    block,
                ),
                format!("\"{after}\" ({t})"),
                "{what}: the deleted node is said by its last label"
            );
        }
    }

    /// **A present node heads the pane as the landed document holds
    /// it**, as the face header beside it does: a rename that has not
    /// landed is not the heading yet.
    #[test]
    fn a_selected_nodes_heading_says_the_landed_label_while_a_rename_has_not_landed() {
        let tol = pncad::tolerance::witness();
        let (doc, block, _) = crate::test_support::boss_on_block("heading-landed", tol);
        let t = test_utils::refusal::tag(block.0.digest());
        let mut session = DocSession::inline(doc, tol);
        let label = |name: &str| SessionOp::SetLabel {
            node: block,
            label: Some(Label::new(name).expect("a label")),
        };
        assert!(session.perform(label("landed")).refusal.is_none());
        session.pump();
        session.perform(SessionOp::Select(Selection::Node(block)));
        assert!(session.perform(label("shown")).refusal.is_none());
        assert_eq!(
            super::selected_node_heading(
                session.landed_pair().map(|(doc, _)| doc),
                &crate::parts::PartFiles::default(),
                session.selection_said(),
                block,
            ),
            format!("\"landed\" ({t})"),
            "the heading waits for the rename to land"
        );
    }

    /// **A name that no longer resolves is a verdict to act on**, so
    /// it is drawn in the actionable colour — and the rebind count
    /// under it is secondary text, weak.
    #[test]
    fn a_vanished_faces_verdict_is_drawn_loud_and_its_offer_count_weak() {
        let (painted, voices) = drawn(&face(Some(vanished(vec![name(EntityKind::Face)]))));
        assert_eq!(
            find_opening(&painted, "this face is stranded: ").ink,
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
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(2)),
                body: 0,
            },
            resolution: Some(Box::new(Resolution::Indeterminate(ResolveIndeterminate {
                standing: NodeStanding::Failed {
                    node: RecipeNodeId::new(0, test_utils::refusal::tagged(1)),
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

    /// **A deleted node's one word is loud**, beside its name.
    #[test]
    fn a_deleted_nodes_verdict_is_drawn_loud() {
        let (painted, voices) = drawn(&Standing::Node {
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(3)),
            present: false,
        });
        assert_eq!(find(&painted, "deleted").ink, Some(voices.actionable));
    }

    /// **An undeclared variable is said once, loud** — the only line
    /// the pane draws for it (`properties_ui`'s `Variable` arm draws none)
    /// — and in the refusal vocabulary's words: the line is
    /// `Refusal::undeclared_wording`, which `Refusal::NoSuchVariable`
    /// opens with, so the pane and a drag's refusal cannot drift apart.
    #[test]
    fn an_undeclared_variables_verdict_is_drawn_loud() {
        let var = SpokenVar::new(VarId::new(0, 9), Some(VarName::from_static("width")));
        let said = crate::session::Refusal::undeclared_wording(&var);
        assert_eq!(said, "variable width is not in this document");
        let (painted, voices) = drawn(&Standing::Variable {
            var,
            present: false,
        });
        assert_eq!(find(&painted, &said).ink, Some(voices.actionable));
    }

    /// **A selection that still denotes has no verdict**, so nothing
    /// is said at all — not a quiet "fine".
    #[test]
    fn a_standing_that_still_denotes_says_nothing() {
        for standing in [
            Standing::Empty,
            Standing::Node {
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(3)),
                present: true,
            },
            Standing::Variable {
                var: SpokenVar::new(VarId::new(0, 9), Some(VarName::from_static("width"))),
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
