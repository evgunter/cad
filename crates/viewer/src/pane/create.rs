//! The creation forms: the modal tools and the add-a-node panels the
//! Properties pane hosts above the selection.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use std::collections::BTreeMap;

use eframe::egui;
use pncad::document::{AxisSense, BooleanOp, DocumentId, MatePrimitive, RecipeNodeId};
use pncad::select::SplitHalf;

use crate::app::ViewerBehavior;
use crate::blend::{BlendError, BlendKindChoice, BlendTarget, FREEZE_NOTE};
use crate::combine::{DUPLICATE_GAP, PatternOutputChoice, STEP_DIRECTION};
use crate::drafts::{CommitFault, Drafts, scalars};
use crate::forms::{
    ANGLE_DRAG_SPEED, COUNT_DRAG_SPEED, DatumKindChoice, FIELD_DRAG_SPEED, MATE_PRIMITIVES,
    PartSelectChoice, PatternKindChoice, ShapeEdits, ShapeKind, UNIT_DRAG_SPEED, boolean_op_label,
    split_half_label,
};
use crate::frame::{self, Tone};
use crate::matetool::{MateChoice, MateToolState, admitted_classes};
use crate::pane::profile::{notation_row, path_steps_ui, preview_verdict};
use crate::parts::{PartChooser, PartEntry};
use crate::props::render_number;
use crate::seats::{Seats, seat_line};
use crate::session::{
    FaceFrameFault, FaceSelection, ProfilePlane, Selection, SessionOp, Standing, face_frame_seat,
};
use crate::sketch;
use crate::theme::Theme;
use crate::tools::ToolKind;
use crate::tree;
use crate::widgets::{
    angle_picker, length_picker, number_field, point_fields, unit_field, unit_vec3_row, vec3_row,
};

/// **The smallest instance index the part form offers.**
///
/// A pattern's instances are indexed from zero, and an index below it
/// refuses at evaluation (`InstanceOutOfRange`, typed, on the node's
/// own badge), so the form declines to author one — the same rule
/// [`MIN_PATTERN_COUNT`] follows. There is deliberately no upper
/// bound: how many instances the pattern has is a fact about its
/// VALUE, not about the document, so a cap here would be a limit read
/// off a picture that can change under it.
pub(crate) const MIN_PART_INSTANCE: i64 = 0;

/// **What a projection does to the picture**, said on the panel
/// before the click: the split or pattern it reads stops being a root,
/// so every body it does NOT select stops being drawn.
///
/// Not a refusal and not a surprise to hide — it is the document's
/// roots rule (a new node takes its inputs' place), and a person who
/// wants the other bodies kept projects each one, which is what the
/// duplicate tool does for a pattern of two.
///
/// **It names the feature tree, because that is the only place left.**
/// Once the split or pattern has left the picture, a viewport click
/// meets the PROJECTION's body, and the projection tool seats the drawn
/// body ([`crate::session::Selection::seat_node`]) — which is not a
/// split or a pattern. Re-reaching the source from the viewport is
/// `work/forms/a-projected-split-is-unreachable-from-the-viewport`.
pub(crate) const PROJECTION_HIDES_THE_REST: &str = "only the selected body stays drawn: the split or pattern it is read out of leaves the \
     picture — to project another of its bodies, pick it again in the feature tree";

/// **The part form's selector rows**: which of the two selections is
/// being authored, the one field or radio row that selection needs,
/// and what committing it does to the picture
/// ([`PROJECTION_HIDES_THE_REST`]).
///
/// A free function over the `Ui` for [`profile_plane_row`]'s reason —
/// `ViewerBehavior` borrows the whole application, so this is the only
/// seam a headless row can drive, and the method's job is to call it.
pub(crate) fn part_selector_rows(
    ui: &mut egui::Ui,
    theme: &Theme,
    select: &mut PartSelectChoice,
    half: &mut SplitHalf,
    instance: &mut i64,
) {
    ui.horizontal(|ui| {
        ui.label("select");
        for (choice, label) in PartSelectChoice::ALL {
            ui.radio_value(select, choice, label);
        }
    });
    match select {
        PartSelectChoice::Half => {
            ui.horizontal(|ui| {
                ui.label("half");
                // One button per half the KERNEL has, in its order:
                // the form offers the vocabulary, never a copy of it.
                for side in SplitHalf::ALL {
                    ui.radio_value(half, side, split_half_label(side));
                }
            });
            crate::widgets::message_toned(
                ui,
                "the tool plane's normal side is above",
                theme,
                Tone::Advisory,
            );
        }
        PartSelectChoice::Instance => {
            ui.horizontal(|ui| {
                ui.label("instance");
                ui.add(
                    number_field(instance, COUNT_DRAG_SPEED).range(MIN_PART_INSTANCE..=i64::MAX),
                );
            });
            crate::widgets::message_toned(
                ui,
                "instances are numbered from zero, in placement order",
                theme,
                Tone::Advisory,
            );
        }
    }
    crate::widgets::message_toned(ui, PROJECTION_HIDES_THE_REST, theme, Tone::Advisory);
}

/// **What the duplicate tool tells a user before they click it** —
/// where the copy will land, which is
/// [`crate::combine::duplicate_step`]'s RULE: the
/// step itself is measured off the body at the commit, so what the
/// panel can promise ahead of it is how the step is chosen, and that
/// the copy clears the original.
///
/// A free function answering a STRING rather than painting, so a
/// headless row can read the sentence and the panel can only show what
/// this composed. Its numbers go through [`render_number`], the
/// crate's one spelling of a number a person reads.
pub(crate) fn duplicate_note() -> String {
    let [x, y, z] = STEP_DIRECTION.map(render_number);
    format!(
        "the copy lands along ({x}, {y}, {z}), clear of the original by at least {}% of the \
         body's width — more for a body thin that way — measured off the body as it now is; \
         every slot is editable afterwards",
        render_number(DUPLICATE_GAP * 100.0),
    )
}

/// **A seated tool's held picks, drawn**: [`seat_line`] over the
/// tool's own [`Seats`], in the advisory voice every tool panel says
/// its picks in.
///
/// Takes the `Seats` rather than a line or a list of roles, so a panel
/// has no roles to re-list and no line to compose; a free function over
/// the `Ui` so a headless row can read what it paints
/// (`crate::pane::headless`).
pub(crate) fn seats_row(ui: &mut egui::Ui, seats: &Seats, theme: &Theme) {
    crate::widgets::message_toned(ui, seat_line(seats), theme, Tone::Advisory);
}

/// **The mate tool's held picks, drawn** — [`MateToolState::line`],
/// the seated tools' line over the mate's two sides, in the same voice
/// as [`seats_row`].
pub(crate) fn mate_picks_row(ui: &mut egui::Ui, state: &MateToolState, theme: &Theme) {
    crate::widgets::message_toned(ui, state.line(), theme, Tone::Advisory);
}

/// **The smallest pattern count the form offers.**
///
/// A pattern of zero instances refuses at evaluation
/// (`NonPositiveCount`, typed, on the node's own badge), so the form
/// declines to author one — the same rule the bore field follows. There
/// is deliberately NO upper bound: the property panel imposes none on
/// the slot afterwards, and a cap here would be a limit the document
/// does not have.
pub(crate) const MIN_PATTERN_COUNT: i64 = 1;

/// **A form's frame pick**: a combo over the choices the form offers,
/// or a line saying there are none. One widget for every form that
/// writes against a frame, so they name frames one way.
///
/// `text` is a TOTAL function of a choice rather than a label stored
/// beside it, because the closed combo has to name a pick the list no
/// longer offers: a held pick outlives the frame it names (the
/// document swap row,
/// `work/forms/a-creation-forms-held-pick-survives-a-document-swap`),
/// and a picker that fell silent there would read as no pick while the
/// commit still carried one.
fn frame_picker<T: Copy + PartialEq>(
    ui: &mut egui::Ui,
    theme: &Theme,
    label: &str,
    salt: &str,
    choices: &[T],
    picked: &mut Option<T>,
    text: impl Fn(&T) -> String,
) {
    if choices.is_empty() {
        // A sentence, so a line of its own under the label rather than
        // the rest of the label's row.
        ui.label(label);
        crate::widgets::message_toned(ui, NO_FRAMES, theme, Tone::Advisory);
        return;
    }
    ui.horizontal(|ui| {
        ui.label(label);
        let shown = picked.as_ref().map_or_else(|| "pick one".to_owned(), &text);
        egui::ComboBox::from_id_salt(salt)
            .selected_text(shown)
            .show_ui(ui, |ui| {
                for choice in choices {
                    ui.selectable_value(picked, Some(*choice), text(choice));
                }
            });
    });
}

/// What [`frame_picker`] says in place of a combo with nothing to offer.
const NO_FRAMES: &str = "none in this document — add a frame datum first";

/// **The "Add part" chooser's window**, as wide as the pane its door
/// is drawn in (`opener`) and never under
/// [`crate::widgets::message_floor`].
///
/// The width is given rather than left to egui's own default for a
/// window, because it is what every sentence in the chooser wraps at:
/// the chooser answers the pane that opened it, so it lays its
/// sentences out the way that pane would.
///
/// **Pinned, every frame**, as both the least and the most the window
/// may be. A `default_width` alone is read once per session — egui
/// persists a window's size under its id and only ratchets it up from
/// there — so a chooser reopened after the pane narrowed would keep
/// the old width. Pinning also follows a pane resized while the
/// chooser is open.
fn part_window(opener: &egui::Ui) -> egui::Window<'static> {
    let width = opener
        .available_width()
        .max(crate::widgets::message_floor(opener));
    egui::Window::new("Add part")
        .collapsible(false)
        .resizable(false)
        .min_width(width)
        .max_width(width)
}

/// **One part the chooser offers**: its pick button, and the
/// document's id on a line of its own under it. Answers whether the
/// button was clicked.
///
/// The id is a VALUE, not a sentence: 32 hex digits, bounded by its
/// source and with no space to break at, so wrapping it at the region
/// would split one id into two tokens. It is drawn whole or, in a
/// window narrower than it, elided with the whole id on hover —
/// egui's own truncation, which never breaks it. The file name above
/// it is what a reader picks by (`crate::parts::catalogue` sorts on
/// it); the id tells two same-named files apart.
///
/// An entry that cannot be picked stays VISIBLE and disabled, carrying
/// the op's own refusal — read off the entry, not minted here.
fn part_entry(ui: &mut egui::Ui, theme: &Theme, entry: &PartEntry) -> bool {
    let refusal = entry.refusal();
    let picked = crate::app::refusable_button(ui, entry.file_name(), refusal.as_ref());
    ui.add(
        egui::Label::new(crate::app::toned(
            entry.id.to_string(),
            theme,
            Tone::Advisory,
        ))
        .truncate(),
    );
    picked
}

/// **What the add-profile form calls the frame it offers to mint.**
///
/// A choice in the same combo as the document's own frames, because
/// that is the question being answered — which frame — and a document
/// with none is the case this entry exists for.
const NEW_XY_LABEL: &str = "a new xy frame";

/// **The plane choices the add-profile form offers**, in the order it
/// offers them: the mint, then the document's own frames.
///
/// Its own function so that "the mint goes first, whatever the
/// document holds" is a claim a test can read directly as well as
/// through the widget.
fn profile_plane_choices(frames: &[RecipeNodeId]) -> Vec<ProfilePlane> {
    core::iter::once(ProfilePlane::NewXy)
        .chain(frames.iter().copied().map(ProfilePlane::Existing))
        .collect()
}

/// **The add-profile form's plane row**: the mint, then the document's
/// own frames, in one combo.
///
/// A free function over the `Ui` rather than a private arm of
/// [`ViewerBehavior::add_profile_ui`], because it is the only seam a
/// headless test can DRIVE: `ViewerBehavior` borrows the whole
/// application, and a labelling helper proved correct beside the panel
/// is not a labelling helper the panel calls. `create::tests` runs this
/// against a real `egui::Context` and reads the text it painted.
///
/// **The mint goes FIRST and unconditionally**, which is what retires
/// the "none in this document — add a frame datum first" dead end at
/// this form: the question the combo answers is *which frame*, and an
/// empty document's honest answer to it is *a new one*. The list here
/// is therefore never empty, so [`frame_picker`]'s empty line is
/// reachable only from the add-datum form, whose axis-in-sketch kind
/// genuinely does need a frame that already exists.
pub(crate) fn profile_plane_row(
    ui: &mut egui::Ui,
    theme: &Theme,
    frames: &[RecipeNodeId],
    names: &impl Fn(&RecipeNodeId) -> String,
    picked: &mut Option<ProfilePlane>,
) {
    let planes = profile_plane_choices(frames);
    frame_picker(
        ui,
        theme,
        "on frame",
        "profile_plane",
        &planes,
        picked,
        |plane| match plane {
            ProfilePlane::NewXy => NEW_XY_LABEL.to_owned(),
            ProfilePlane::Existing(id) => names(id),
        },
    );
}

/// **The chooser's answer, drawn**: the directory it read, then the
/// parts on offer or what it has to say instead — and the entry that
/// was clicked, when one was.
///
/// How loud the answer is, is the chooser's to say
/// ([`PartChooser::tone`]), read once for the two arms that draw a
/// sentence rather than a list.
///
/// A chooser with no directory draws no header line: it has refused
/// for exactly that reason ([`crate::session::Refusal::NoDocumentDirectory`],
/// minted from the same resolver its directory is), and a quiet "no
/// directory" above the loud refusal would be one fact in two voices.
///
/// A free function over the `Ui` so a headless drive can reach it
/// (`crate::pane::headless`).
fn part_listing(ui: &mut egui::Ui, theme: &Theme, chooser: &PartChooser) -> Option<DocumentId> {
    if let Some(dir) = chooser.dir() {
        crate::widgets::message_toned(
            ui,
            format!("parts in {}", dir.display()),
            theme,
            Tone::Advisory,
        );
    }
    let mut chosen = None;
    match chooser.offered() {
        // An EMPTY listing is not "no parts here": `PartChooser::tone`
        // says why, and why it is loud. The sentence says it to the
        // reader.
        Ok([]) => {
            crate::widgets::message_toned(
                ui,
                "this directory holds no documents at all — not even the open document's own \
                 file, which has gone from it",
                theme,
                chooser.tone(),
            );
        }
        Ok(entries) => {
            for entry in entries {
                if part_entry(ui, theme, entry) {
                    chosen = Some(entry.id);
                }
            }
        }
        // The refusing layer's own sentence — the store's or the
        // directory rule's — never one composed here.
        Err(refusal) => {
            crate::widgets::message_toned(ui, refusal.to_string(), theme, chooser.tone());
        }
    }
    chosen
}

/// **A face-frame fault under the datum form**, in the voice the fault
/// gives itself ([`FaceFrameFault::tone`]) — or nothing, where this
/// pane already says the fact elsewhere, because it says a fact once:
///
/// - `NoFace` is the form's unmet seat, which the form has already
///   asked for in the same words from its one home;
/// - `Unresolved`, when `said_by_selection`: the form's latched face IS
///   the selection, and the selection's own verdict
///   (`pane::properties::standing_verdict`, in this pane) is already
///   saying, loud, that it does not resolve. A latched face that is no
///   longer selected has nobody else to say it, so the form does.
///
/// A free function over the `Ui` so a headless drive can reach it.
fn face_frame_fault(
    ui: &mut egui::Ui,
    theme: &Theme,
    fault: &FaceFrameFault,
    said_by_selection: bool,
) {
    let said_elsewhere = match fault {
        FaceFrameFault::NoFace => true,
        FaceFrameFault::Unresolved { .. } => said_by_selection,
        FaceFrameFault::NotLanded
        | FaceFrameFault::NotOneBody { .. }
        | FaceFrameFault::NotPlanar { .. } => false,
    };
    if !said_elsewhere {
        crate::widgets::message_toned(ui, fault.to_string(), theme, fault.tone());
    }
}

/// **Whether the selection's verdict is already saying that the form's
/// latched face does not resolve**: the latched face is the one
/// selected, and the selection carries an unresolved verdict
/// ([`Standing::unresolved`]). A selected face that resolves, or has no
/// evaluation behind it yet, says no such thing, so the form's own
/// refusal is the only place a reader would learn it.
fn selection_says_unresolved(standing: &Standing, latched: Option<&FaceSelection>) -> bool {
    match standing {
        Standing::Face { face, .. } => Some(face) == latched && standing.unresolved().is_some(),
        Standing::Empty
        | Standing::Node { .. }
        | Standing::Param { .. }
        | Standing::Edge { .. } => false,
    }
}

/// **Why the add-profile button is held**, when it is — and how loud
/// that is, which depends on which of two things it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Held {
    /// The form is waiting for its next input: a frame, a shape, a
    /// first step. The form asking, [`Tone::Advisory`].
    Waiting(&'static str),
    /// An input the reader gave is refused, and the button stays shut
    /// until they change it: [`Tone::Actionable`].
    Refused(&'static str),
}

impl Held {
    fn words(self) -> &'static str {
        match self {
            Self::Waiting(words) | Self::Refused(words) => words,
        }
    }

    fn tone(self) -> Tone {
        match self {
            Self::Waiting(_) => Tone::Advisory,
            Self::Refused(_) => Tone::Actionable,
        }
    }
}

/// **What a bored circle holds the button for**: a bore at least as
/// wide as the radius, which is an input the reader gave and the form
/// refuses — [`Held::Refused`], not a request for the next input.
fn bore_held(bored: bool, bore: f64, radius: f64) -> Option<Held> {
    (bored && bore >= radius).then_some(Held::Refused(
        "the bore must be smaller than the radius — which loop is the hole is decided by \
         containment, so a larger bore would swap the roles rather than refuse",
    ))
}

/// The held button's reason, drawn in its own voice. A free function
/// over the `Ui` so a headless drive can reach it.
fn held_line(ui: &mut egui::Ui, theme: &Theme, held: Held) {
    crate::widgets::message_toned(ui, held.words(), theme, held.tone());
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
            ui.separator();
            // Beside the pattern they read and author: a projection
            // takes ONE body out of a split's or a pattern's several,
            // and a duplicate authors a pattern of two with both
            // projections already made.
            self.projection_tool_ui(ui);
            ui.separator();
            self.duplicate_tool_ui(ui);
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
        crate::widgets::message(ui, ToolKind::Mate.says(&"pick two faces in the viewport"));
        mate_picks_row(ui, tool.state(), &self.theme);
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
            crate::widgets::message_toned(
                ui,
                format!(
                    "admission: {}",
                    match entry.admission {
                        pncad::document::ClassAdmission::Mints => "mints an at-rest record",
                        other => other.no_record_reason(),
                    }
                ),
                &self.theme,
                Tone::Advisory,
            );
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
            part_window(ui).show(ui.ctx(), |ui| {
                chosen = part_listing(ui, &self.theme, chooser);
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
    /// Four of the six kinds are numbers alone. The two that are not
    /// each take a PICK, from different places: an axis in a sketch
    /// names the frame its coordinates are written in, picked from the
    /// document's frames the way the add-profile form picks its plane,
    /// and a frame on a face names the face itself, picked in the
    /// viewport. Either way the button waits until the pick is in.
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
                crate::widgets::message(ui, "y is squared against x; the normal is x × y");
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
                let names = self.frame_names();
                frame_picker(
                    ui,
                    &self.theme,
                    "in frame",
                    "datum_frame",
                    &frames,
                    &mut self.drafts.datum_frame,
                    names,
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
                crate::widgets::message(
                    ui,
                    "x and y are the frame's own; a revolve needs its profile on this frame",
                );
            }
            DatumKindChoice::FaceFrame => self.datum_face_frame_rows(ui),
            DatumKindChoice::Point => self.datum_origin_row(ui, "position"),
        }
        // The face-frame gate, on the kind that has one, asked ONCE:
        // its `Ok` is the pair the spec is lowered from and its `Err`
        // is the sentence over the held button, so the button is gated
        // by the same computation it commits. A second derivation of
        // the picks would gate on one and commit the other.
        let seat = (kind == DatumKindChoice::FaceFrame)
            .then(|| face_frame_seat(self.session.landed_pair(), self.drafts.datum_face.as_ref()));
        let refused = seat.as_ref().and_then(|seat| seat.as_ref().err());
        // Lowered every frame, so the button's enabling and its commit
        // read one value: `Ok(None)` is a seat still unfilled, and it
        // is what holds the button. The sentence over it follows the
        // KIND — the two picking kinds want different things from
        // different places, so one sentence for the form would be
        // false of whichever is not showing.
        let datum = self
            .drafts
            .datum_spec(seat.as_ref().and_then(|seat| seat.as_ref().ok()));
        let unpicked = matches!(datum, Ok(None));
        if unpicked && let Some(wanted) = kind.unmet_seat() {
            crate::widgets::message_toned(ui, wanted, &self.theme, Tone::Advisory);
        }
        // `NoFace` is the unmet seat above, in the same words from its
        // one home: the sentence asking for the pick is drawn once.
        if let Some(fault) = refused {
            let said = selection_says_unresolved(
                &self.session.standing(),
                self.drafts.datum_face.as_ref(),
            );
            face_frame_fault(ui, &self.theme, fault, said);
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
                // The drawn body a pick is on, in the one sentence
                // this crate names that scope with
                // (`Display for BlendTarget`): a target that grew a
                // third component would name the wrong scope here too.
                Some(face) => ui.weak(BlendTarget::of_face(face).to_string()),
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
        crate::widgets::message(
            ui,
            "sketch +x, turned about the face's outward normal; the origin is the face's",
        );
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

    /// **How a picker names one frame node**: [`tree::node_label`],
    /// against the same landed document [`Self::frames`] listed.
    ///
    /// One reading for the list and for the closed text. A combo
    /// entry and that node's tree row do not read alike — the row
    /// leads with the node's KIND and this leads with its number —
    /// but the half that says WHICH frame is the same string from the
    /// same function ([`tree::frame_pose`]), so the two agree about
    /// the thing they are both trying to tell apart.
    ///
    /// Total, and that is what it is for: a held pick outlives the
    /// frame it names, and an id the landed document does not hold is
    /// named by its number alone — the text every refusal in this
    /// crate calls a node by, and the text this combo drew for every
    /// frame before it drew their poses.
    fn frame_names(&self) -> impl Fn(&RecipeNodeId) -> String + use<> {
        let named: BTreeMap<RecipeNodeId, String> = self
            .session
            .landed_pair()
            .map(|(doc, _)| {
                sketch::frames(doc)
                    .into_iter()
                    .filter_map(|id| doc.node(id).map(|node| (id, tree::node_label(node, id))))
                    .collect()
            })
            .unwrap_or_default();
        move |id| {
            named
                .get(id)
                .cloned()
                .unwrap_or_else(|| tree::node_number(*id))
        }
    }

    /// The add-profile form: a template shape with Length fields, one
    /// [`SessionOp::AddProfile`] on commit — on a frame picked from the
    /// document's frames.
    ///
    /// The circle's optional bore is what lets this template author
    /// the hollow ring's annulus (one profile node, two loops).
    ///
    /// **The plane is a PICK**, and the picker offers the world xy
    /// frame this form would MINT beside every frame the document
    /// already holds ([`ProfilePlane`]) — so an empty document can
    /// author a sketch without a trip to the add-datum form, in one
    /// submit and one undo.
    ///
    /// Drawing on a picked FACE is still two gestures: minting that
    /// face's frame in the add-datum form
    /// ([`DatumKindChoice::FaceFrame`]) and choosing it here.
    /// `work/author/add-profile-placement-on-picked-face-frame.md`
    /// carries that residue.
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
        // **The frame it is drawn on.** A profile's plane is a node,
        // so what this picks is always a node — either one the
        // document holds, or the world xy frame the same submit
        // inserts and the profile then names. Nothing is conjured:
        // the minted frame is an ordinary feature, in the tree and in
        // the property panel.
        profile_plane_row(
            ui,
            &self.theme,
            &self.frames(),
            &self.frame_names(),
            &mut self.drafts.profile_plane,
        );
        let shape = self.drafts.profile_shape;
        let mut blocked: Option<Held> = None;
        // Stated before the shape check so the FIRST thing a person is
        // told is the thing they have to do first.
        if self.drafts.profile_plane.is_none() {
            blocked = Some(Held::Waiting("pick a frame to draw on"));
        }
        match shape {
            // No shape chosen: the form is at rest. It says what it is
            // waiting for and draws nothing — no fields to fill in for
            // a shape nobody picked, and no preview in the viewport.
            None => blocked = blocked.or(Some(Held::Waiting("choose a shape to add"))),
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
                if let Some(held) = bore_held(
                    self.drafts.profile_bored,
                    self.drafts.profile_bore,
                    self.drafts.profile_radius,
                ) {
                    blocked = Some(held);
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
                    blocked = Some(Held::Waiting("add a step to the chain"));
                }
            }
        }
        if let Some(held) = blocked {
            held_line(ui, &self.theme, held);
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
                if ui
                    .button(format!("Extrude {}", tree::node_number(node)))
                    .clicked()
                {
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
        crate::widgets::message(
            ui,
            ToolKind::Revolve.says(&"pick the profile, then the axis"),
        );
        seats_row(ui, tool.seats(), &self.theme);
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
        crate::widgets::message(
            ui,
            ToolKind::Boolean.says(&"pick the first body, then the second"),
        );
        seats_row(ui, tool.seats(), &self.theme);
        ui.horizontal(|ui| {
            ui.label("operation");
            // One button per operation the KERNEL has, in its order:
            // the form offers the vocabulary, never a copy of it.
            for &op in BooleanOp::ALL {
                ui.radio_value(&mut self.drafts.boolean_op, op, boolean_op_label(op));
            }
        });
        if self.drafts.boolean_op == BooleanOp::Subtract {
            crate::widgets::message_toned(
                ui,
                "subtract removes the second pick from the first",
                &self.theme,
                Tone::Advisory,
            );
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
        crate::widgets::message(
            ui,
            ToolKind::Split.says(&"pick the body, then the datum plane"),
        );
        seats_row(ui, tool.seats(), &self.theme);
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
        crate::widgets::message(ui, ToolKind::Transform.says(&"pick the body to place"));
        seats_row(ui, tool.seats(), &self.theme);
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
        crate::widgets::message(
            ui,
            ToolKind::Pattern.says(&"pick the body, then (circular) the axis"),
        );
        seats_row(ui, tool.seats(), &self.theme);
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

    /// The projection tool's panel — [`crate::combine::PartTool`],
    /// which authors a `Node::Part`: one pick of a split or a pattern,
    /// the selector with its field, and the one committed edit.
    ///
    /// Called the PROJECTION tool on screen because "part" is already
    /// the word the `Add part…` chooser beside it uses for another
    /// document, and the two gestures have nothing to do with each
    /// other.
    ///
    /// **Two seats, one pick.** A half is read out of a split and an
    /// index out of a pattern, so the two selections want different
    /// node kinds and the seat machinery routes a click to the seat
    /// only it can fill ([`crate::combine::PartTool`]). The selector
    /// then picks which seat the commit reads, so a user who picked a
    /// pattern and asked for a half is told which pick is missing
    /// rather than having one silently substituted.
    pub(crate) fn projection_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.part() else {
            if ui.button("Projection tool…").clicked() {
                self.tools.open(ToolKind::Part);
            }
            return;
        };
        crate::widgets::message(
            ui,
            ToolKind::Part.says(
                &"pick a split or a pattern, in the viewport or the tree, to project one body out \
                  of",
            ),
        );
        seats_row(ui, tool.seats(), &self.theme);
        part_selector_rows(
            ui,
            &self.theme,
            &mut self.drafts.part_select,
            &mut self.drafts.part_half,
            &mut self.drafts.part_instance,
        );
        self.tool_commit_row(ui, "Commit projection", ToolKind::Part, |drafts| {
            Ok(match drafts.part_select {
                PartSelectChoice::Half => tool.half_op(drafts.part_half)?,
                PartSelectChoice::Instance => tool.instance_op(drafts.part_instance)?,
            })
        });
    }

    /// The duplicate tool's panel: one body pick, the sentence saying
    /// where the copy lands, and the one committed edit.
    ///
    /// **No fields, deliberately.** The gesture's whole point is that
    /// the next thing a user does is move the copy, so a form asking
    /// where it should go first would be the pattern form again. What
    /// the panel owes instead is the number it commits without asking,
    /// which [`duplicate_note`] states.
    pub(crate) fn duplicate_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.duplicate() else {
            if ui.button("Duplicate tool…").clicked() {
                self.tools.open(ToolKind::Duplicate);
            }
            return;
        };
        crate::widgets::message(ui, ToolKind::Duplicate.says(&"pick the body to duplicate"));
        seats_row(ui, tool.seats(), &self.theme);
        crate::widgets::message_toned(ui, duplicate_note(), &self.theme, Tone::Advisory);
        self.tool_commit_row(ui, "Commit duplicate", ToolKind::Duplicate, |_| {
            Ok(tool.op()?)
        });
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
        crate::widgets::message(ui, ToolKind::Blend.says(&"pick the edges to blend"));
        crate::widgets::message_toned(ui, FREEZE_NOTE, &self.theme, Tone::Advisory);
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
            if clear_picks_button(ui, count)
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

/// **The blend tool's `Clear picks`**, live while `count` edges are
/// held. No operation stands behind it — clearing is tool state — so
/// with nothing held there is no refusal to read, and the literal here
/// is the sentence: each state carries its own on the hook egui shows
/// in that state.
///
/// Answers whether it was clicked, which a disabled button never is.
fn clear_picks_button(ui: &mut egui::Ui, count: usize) -> bool {
    let button = ui.add_enabled(count > 0, egui::Button::new("Clear picks"));
    if count > 0 {
        button
            .on_hover_text("drop every picked edge and start on any body")
            .clicked()
    } else {
        button
            .on_disabled_hover_text("no edge is picked, so there is nothing to clear")
            .clicked()
    }
}

/// **The creation forms' own widgets, driven** —
/// `crate::pane::headless` carries the harness and what it can and
/// cannot reach.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::document::{Doc, ProfileProgram, RecipeNodeId};
    use pncad::geom_core::Tol;
    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
    use pncad::select::SplitHalf;

    use super::{
        NEW_XY_LABEL, ProfilePlane, clear_picks_button, duplicate_note, mate_picks_row,
        part_selector_rows, profile_plane_row, seats_row,
    };
    use crate::combine::BooleanTool;
    use crate::forms::PartSelectChoice;
    use crate::matetool::MateToolState;
    use crate::pane::headless::{painted_after_clicking, painted_text, painted_while_hovering};
    use crate::session::FaceSelection;
    use crate::theme::Theme;

    /// A face pick on the body of `node`.
    fn face_on(node: u64) -> FaceSelection {
        FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(node),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node: RecipeNodeId(node),
            body: 0,
        }
    }

    /// **The mate panel's picks, painted**: the seated tools' line —
    /// one `role: pick` item per side, `—` for the open one, the
    /// seated line's empty sentence — with a pick called the face of a
    /// FEATURE, which is what every other panel calls a node.
    #[test]
    fn the_mate_panel_says_its_picks_in_the_seated_panels_line() {
        let painted =
            |state: &MateToolState| painted_text(|ui| mate_picks_row(ui, state, &Theme::DEFAULT));
        assert_eq!(painted(&MateToolState::Idle), "no picks yet");
        assert_eq!(
            painted(&MateToolState::One(face_on(3))),
            "pick a: face of feature 3; pick b: —"
        );
        assert_eq!(
            painted(&MateToolState::Two {
                a: face_on(3),
                b: face_on(5),
            }),
            "pick a: face of feature 3; pick b: face of feature 5"
        );
    }

    /// **A seated panel's picks, painted in its tool's role order**:
    /// the boolean's first pick is the operand a subtraction KEEPS,
    /// and the line says so before the second is picked.
    #[test]
    fn the_boolean_panel_says_which_operand_each_pick_is() {
        let doc = Doc::<ProfileProgram>::empty_derived("seats-row", Tol::witness());
        let mut tool = BooleanTool::new();
        let painted =
            |tool: &BooleanTool| painted_text(|ui| seats_row(ui, tool.seats(), &Theme::DEFAULT));
        assert_eq!(painted(&tool), "no picks yet");
        tool.pick(&doc, RecipeNodeId(3));
        assert_eq!(
            painted(&tool),
            "first operand: feature 3; second operand: —"
        );
        tool.pick(&doc, RecipeNodeId(5));
        assert_eq!(
            painted(&tool),
            "first operand: feature 3; second operand: feature 5"
        );
    }

    /// The part form's selector rows, driven: the half choice paints
    /// the kernel's own two sides and no index field.
    ///
    /// The row exists because the two selections are one form: a form
    /// that painted both a half choice and an index would be offering
    /// a pairing no node has, and the selector is the only thing
    /// keeping them apart.
    #[test]
    fn the_part_form_paints_the_halves_when_a_half_is_selected() {
        let (mut select, mut half, mut instance) =
            (PartSelectChoice::Half, SplitHalf::Above, 1_i64);
        let drawn = painted_text(|ui| {
            part_selector_rows(ui, &Theme::DEFAULT, &mut select, &mut half, &mut instance)
        });
        assert!(drawn.contains("half"), "{drawn}");
        assert!(
            drawn.contains("above") && drawn.contains("below"),
            "{drawn}"
        );
        assert!(
            !drawn.contains("numbered from zero"),
            "the index field's own sentence is not painted under the half choice: {drawn}"
        );
    }

    /// And the index choice paints the field with its numbering
    /// sentence, and no half radios.
    #[test]
    fn the_part_form_paints_the_index_when_an_instance_is_selected() {
        let (mut select, mut half, mut instance) =
            (PartSelectChoice::Instance, SplitHalf::Above, 3_i64);
        let drawn = painted_text(|ui| {
            part_selector_rows(ui, &Theme::DEFAULT, &mut select, &mut half, &mut instance)
        });
        assert!(drawn.contains("instance"), "{drawn}");
        assert!(
            drawn.contains("numbered from zero"),
            "the field says where the numbering starts: {drawn}"
        );
        assert!(
            !drawn.contains("the tool plane's normal side"),
            "the half choice's own sentence is not painted here: {drawn}"
        );
    }

    /// **The selector row actually switches the form**: clicking the
    /// instance radio leaves the index field painted where the half
    /// radios were.
    ///
    /// Drives the widget rather than the enum, because a radio row
    /// that painted the right labels and wrote to nothing would pass
    /// every assertion above.
    #[test]
    fn clicking_the_instance_selector_opens_the_index_field() {
        let (mut select, mut half, mut instance) =
            (PartSelectChoice::Half, SplitHalf::Above, 1_i64);
        let drawn = painted_after_clicking("instance of a pattern", |ui| {
            part_selector_rows(ui, &Theme::DEFAULT, &mut select, &mut half, &mut instance);
        });
        assert_eq!(
            select,
            PartSelectChoice::Instance,
            "the click wrote: {drawn}"
        );
        assert!(
            drawn.contains("numbered from zero"),
            "and the form now paints the index field: {drawn}"
        );
    }

    /// **The duplicate panel says how the copy's step is chosen** —
    /// along which direction and by how much clear space — in the
    /// numbers the step rule actually uses.
    ///
    /// Held to the landed node from the other side by
    /// `combine_ops::a_duplicate_keeps_the_notes_promise`, which
    /// asserts the committed pattern's spacing against the same rule.
    #[test]
    fn the_duplicate_note_says_how_the_step_is_chosen() {
        let note = duplicate_note();
        assert!(note.contains("(1, 0, 0)"), "along world +x: {note}");
        assert!(
            note.contains("at least 25% of the body's width"),
            "a quarter of the body's width, as a floor: {note}"
        );
        assert!(
            note.contains("measured off the body as it now is"),
            "and that the step is the body's, not a fixed length: {note}"
        );
        assert!(
            note.contains("editable afterwards"),
            "and that the slot is not frozen: {note}"
        );
    }

    /// **The projection panel says what a projection does to the
    /// picture**, under either selector — the bodies it does not select
    /// stop being drawn, and a person should hear that before the
    /// click, not discover it after.
    #[test]
    fn the_part_form_says_the_other_bodies_leave_the_picture() {
        for choice in [PartSelectChoice::Half, PartSelectChoice::Instance] {
            let (mut select, mut half, mut instance) = (choice, SplitHalf::Above, 1_i64);
            let drawn = painted_text(|ui| {
                part_selector_rows(ui, &Theme::DEFAULT, &mut select, &mut half, &mut instance);
            });
            assert!(
                drawn.contains("only the selected body stays drawn"),
                "{choice:?}: {drawn}"
            );
            assert!(
                drawn.contains("pick it again in the feature tree"),
                "and where the rest can still be reached from: {drawn}"
            );
        }
    }

    /// A stand-in labeller: the number alone, so a row asserting on
    /// the pose half is asserting on text this closure did not write.
    fn numbers(id: &RecipeNodeId) -> String {
        format!("feature {}", id.0)
    }

    /// The add-profile form's plane row, on an EMPTY document, offers
    /// the mint and does not say the document is a dead end.
    ///
    /// The row this unit is filed against is that "on frame — none in
    /// this document — add a frame datum first" is true and useless.
    /// A drive of the widget is what can claim the sentence is gone,
    /// because the sentence lives inside a widget call.
    #[test]
    fn the_plane_row_offers_a_new_frame_when_the_document_holds_none() {
        let mut picked: Option<ProfilePlane> = None;
        let drawn =
            painted_text(|ui| profile_plane_row(ui, &Theme::DEFAULT, &[], &numbers, &mut picked));
        assert!(drawn.contains("on frame"), "{drawn}");
        assert!(
            !drawn.contains("add a frame datum first"),
            "the empty-document dead end is unreachable from this form: {drawn}"
        );
        assert!(
            drawn.contains("pick one"),
            "an unfilled pick still asks to be filled: {drawn}"
        );
    }

    /// **The mint is offered in a document that already holds frames
    /// too** — `profile_plane_row`'s "FIRST and unconditionally".
    ///
    /// The three rows above pass an empty frame list, so between them
    /// they hold only `frame_picker`'s empty-list branch: a mint
    /// offered only when the document has nothing else would satisfy
    /// all of them. This row is the one that does not, and it reads
    /// the OPEN combo's entries rather than its closed text, since
    /// with nothing picked the mint is a choice on the list and not
    /// the selection.
    #[test]
    fn the_plane_row_offers_the_mint_beside_the_frames_that_exist() {
        let frames = [RecipeNodeId(2), RecipeNodeId(5)];
        let mut picked: Option<ProfilePlane> = None;
        let drawn = painted_after_clicking("pick one", |ui| {
            profile_plane_row(ui, &Theme::DEFAULT, &frames, &numbers, &mut picked);
        });
        assert!(
            drawn.contains(NEW_XY_LABEL),
            "the mint is on the open list beside the frames that exist: {drawn}"
        );
        assert!(drawn.contains("feature 2"), "{drawn}");
        assert!(drawn.contains("feature 5"), "{drawn}");
    }

    /// The mint's own name reaches the widget: with it picked, the
    /// closed combo says so rather than naming a node number.
    #[test]
    fn the_plane_row_names_the_frame_it_would_mint() {
        let mut picked = Some(ProfilePlane::NewXy);
        let drawn =
            painted_text(|ui| profile_plane_row(ui, &Theme::DEFAULT, &[], &numbers, &mut picked));
        assert!(drawn.contains(NEW_XY_LABEL), "{drawn}");
    }

    /// **The row draws the LABELLING FUNCTION's sentence**, not a node
    /// number of its own.
    ///
    /// The one assertion that goes red if `profile_plane_row` stops
    /// calling the names it is handed — which is the failure mode a
    /// test of the labelling function alone cannot see.
    #[test]
    fn the_plane_row_draws_the_name_it_is_handed() {
        let mut picked = Some(ProfilePlane::Existing(RecipeNodeId(4)));
        let names = |id: &RecipeNodeId| format!("feature {} — xy at (0, 0, 0) m", id.0);
        let drawn = painted_text(|ui| {
            profile_plane_row(ui, &Theme::DEFAULT, &[RecipeNodeId(4)], &names, &mut picked)
        });
        assert!(
            drawn.contains("feature 4 — xy at (0, 0, 0) m"),
            "the closed combo says which frame, in the labeller's words: {drawn}"
        );
    }

    /// A pick the list no longer offers is still NAMED — the held-pick
    /// case `frame_picker`'s `text` argument exists for.
    #[test]
    fn the_plane_row_names_a_pick_the_document_no_longer_holds() {
        let mut picked = Some(ProfilePlane::Existing(RecipeNodeId(9)));
        let drawn =
            painted_text(|ui| profile_plane_row(ui, &Theme::DEFAULT, &[], &numbers, &mut picked));
        assert!(
            drawn.contains("feature 9"),
            "a pick outside the list is named, not silently drawn as unfilled: {drawn}"
        );
        assert!(!drawn.contains("pick one"), "{drawn}");
    }

    /// **`Clear picks` says why while it is disabled** — hovered with
    /// nothing held, it paints its own sentence, and not the enabled
    /// one that egui would have shown nobody.
    #[test]
    fn clear_picks_with_nothing_held_says_there_is_nothing_to_clear() {
        let hovered = painted_while_hovering("Clear picks", 0, |ui| {
            clear_picks_button(ui, 0);
        });
        assert!(
            hovered.contains("no edge is picked, so there is nothing to clear"),
            "{hovered}"
        );
        assert!(!hovered.contains("drop every picked edge"), "{hovered}");
    }

    /// And holding picks, the same hover paints what a click does.
    #[test]
    fn clear_picks_with_picks_held_says_what_it_drops() {
        let hovered = painted_while_hovering("Clear picks", 0, |ui| {
            clear_picks_button(ui, 3);
        });
        assert!(
            hovered.contains("drop every picked edge and start on any body"),
            "{hovered}"
        );
        assert!(!hovered.contains("nothing to clear"), "{hovered}");
    }
}

/// **Where this pane's sentences LAND** — each measured in a pane
/// narrower than the sentence, through `crate::pane::headless`, as
/// `crate::pane::properties`'s `layout_tests` measure that pane's.
///
/// A sentence drawn beside a control in a `ui.horizontal` is laid out
/// from the control's right-hand edge at infinite width; each row here
/// holds that it is on a line of its own instead, under the control it
/// is about.
#[cfg(test)]
mod layout_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use std::path::PathBuf;

    use eframe::egui;
    use pncad::document::{DocumentId, RecipeNodeId};

    use super::{NO_FRAMES, frame_picker, part_entry, part_window};
    use crate::pane::headless::{
        SLACK, assert_inside, assert_own_lines, assert_under, drawn_in, find, landed_after,
    };
    use crate::parts::PartEntry;
    use crate::theme::Theme;

    /// A narrow pane, and wider than `crate::widgets::message_floor`,
    /// so these rows read the region and not the floor.
    const REGION: f32 = 260.0;

    #[test]
    fn a_frame_pickers_empty_line_is_said_under_its_label_inside_the_pane() {
        let mut picked: Option<RecipeNodeId> = None;
        let (region, painted) = drawn_in(REGION, |ui| {
            frame_picker(
                ui,
                &Theme::DEFAULT,
                "in frame",
                "salt",
                &[],
                &mut picked,
                |id| format!("feature {}", id.0),
            );
        });
        let empty = find(&painted, NO_FRAMES);
        assert_own_lines(region, empty);
        assert_under(find(&painted, "in frame"), empty);
    }

    #[test]
    fn a_parts_id_is_said_under_its_pick_button_inside_the_pane() {
        let entry = PartEntry {
            id: DocumentId(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef),
            path: PathBuf::from("outer-enclosure-lid.pncad"),
            open_document: false,
        };
        let (region, painted) = drawn_in(REGION, |ui| {
            part_entry(ui, &Theme::DEFAULT, &entry);
        });
        let id = find(&painted, &entry.id.to_string());
        assert_own_lines(region, id);
        assert_under(find(&painted, &entry.file_name()), id);
    }

    /// **A part's id is never broken inside itself**: in a pane
    /// narrower than its 32 digits it is elided on one line, where a
    /// sentence's wrap would split it into two tokens.
    #[test]
    fn a_parts_id_is_drawn_on_one_line_in_a_pane_narrower_than_it() {
        const NARROW: f32 = 200.0;
        let entry = PartEntry {
            id: DocumentId(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef),
            path: PathBuf::from("lid.pncad"),
            open_document: false,
        };
        let (region, painted) = drawn_in(NARROW, |ui| {
            part_entry(ui, &Theme::DEFAULT, &entry);
        });
        let id = find(&painted, &entry.id.to_string());
        assert_eq!(
            id.rows.len(),
            1,
            "the id is one line, not two ({:?})",
            id.rows
        );
        assert_inside(region, id);
    }

    /// **The chooser's window takes the width of the pane that opens
    /// it, each time it is opened** — at least that width in a wide
    /// pane, and in a narrow one reopened after a wide one, no more
    /// than that width, so its sentences wrap there.
    ///
    /// One context across both opens, because what egui carries from
    /// the first open to the second is its memory of the window's
    /// size. Two frames per open, because a window paints nothing on
    /// the frame it first appears.
    #[test]
    fn the_part_chooser_takes_the_width_of_the_pane_each_time_it_opens() {
        const WIDE: f32 = 600.0;
        let sentence = "this directory holds no documents at all — not even the open \
                        document's own file, which has gone from it";
        // (opener width, or `None` for a frame with the chooser closed)
        let frames = [Some(WIDE), Some(WIDE), None, Some(REGION), Some(REGION)];
        let widths = core::cell::RefCell::new(Vec::new());
        let mut frame = 0;
        let painted = landed_after(frames.len(), |ui| {
            if let Some(Some(width)) = frames.get(frame) {
                ui.allocate_ui(egui::vec2(*width, 800.0), |opener| {
                    let shown = part_window(opener).show(opener.ctx(), |ui| {
                        crate::widgets::message(ui, sentence);
                    });
                    let drawn = shown.map_or(f32::NAN, |shown| shown.response.rect.width());
                    widths.borrow_mut().push((*width, drawn));
                });
            }
            frame += 1;
        });
        for (opener, drawn) in widths.borrow().iter().skip(1) {
            assert!(
                (drawn - opener).abs() <= SLACK,
                "a chooser opened from a {opener}-point pane is {drawn} points wide \
                 ({:?})",
                widths.borrow()
            );
        }
        let said = find(&painted, sentence);
        assert!(
            said.rows.len() > 1,
            "the sentence is longer than the pane, so it wraps ({:?})",
            said.rows
        );
        let left = said
            .rows
            .iter()
            .map(egui::Rect::left)
            .fold(f32::INFINITY, f32::min);
        // A pane as wide as the opener's, starting where the window's
        // text does: the window sits wherever egui places it.
        let pane = egui::Rect::from_min_size(egui::pos2(left, 0.0), egui::vec2(REGION, 1.0));
        assert_inside(pane, said);
    }
}

/// **How loud this pane's verdicts are drawn**, read off the paint and
/// held against fixed colours ([`crate::pane::headless::Voices`]),
/// through the free functions the pane calls — so a literal tone put
/// back at a draw site, or a value that answers the wrong tone, turns
/// a row red.
#[cfg(test)]
mod tone_tests {
    use std::path::PathBuf;

    use pncad::document::{DocumentId, RecipeNodeId};
    use pncad::prelude::SurfaceKind;

    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
    use pncad::select::{InterrogateError, Resolution};

    use super::{
        Held, bore_held, face_frame_fault, held_line, part_listing, selection_says_unresolved,
    };
    use crate::pane::headless::{Landed, Voices, find, find_opening, landed_voiced};
    use crate::parts::{PartCensus, PartChooser, PartEntry};
    use crate::session::{FaceFrameFault, FaceSelection, Refusal, Standing};
    use crate::theme::Theme;

    fn listed(
        dir: Option<&str>,
        offered: Result<Vec<PartEntry>, Refusal>,
    ) -> (Vec<Landed>, Voices) {
        let chooser = PartChooser::opened(PartCensus::taken(dir.map(PathBuf::from), offered));
        landed_voiced(|ui| {
            part_listing(ui, &Theme::DEFAULT, &chooser);
        })
    }

    /// **A chooser with no directory says so once, loud** — the
    /// refusal's own sentence, and no quiet "no directory" over it.
    #[test]
    fn a_chooser_with_no_directory_says_so_once_and_loud() {
        let (painted, voices) = listed(None, Err(Refusal::NoDocumentDirectory));
        assert_eq!(
            find_opening(&painted, "save the document first").ink,
            Some(voices.unresolved)
        );
        assert_eq!(
            painted.len(),
            1,
            "one line: {:?}",
            painted.iter().map(|l| &l.text).collect::<Vec<_>>()
        );
    }

    /// **An empty listing is loud**: the open document's own file has
    /// gone, and what is placed will stop resolving. The directory it
    /// read is a report, and quiet.
    #[test]
    fn an_empty_listing_is_drawn_loud_under_a_quiet_header() {
        let (painted, voices) = listed(Some("/parts"), Ok(Vec::new()));
        assert_eq!(
            find_opening(&painted, "this directory holds no documents at all").ink,
            Some(voices.unresolved)
        );
        assert_eq!(find(&painted, "parts in /parts").ink, Some(voices.weak));
    }

    /// **A listing is a report**, and its entries' ids are quiet.
    #[test]
    fn a_listing_is_drawn_quiet() {
        let id = DocumentId(7);
        let (painted, voices) = listed(
            Some("/parts"),
            Ok(vec![PartEntry {
                id,
                path: PathBuf::from("/parts/bracket.pncad"),
                open_document: false,
            }]),
        );
        assert_eq!(find(&painted, "parts in /parts").ink, Some(voices.weak));
        assert_eq!(find(&painted, &id.to_string()).ink, Some(voices.weak));
    }

    fn latched() -> FaceSelection {
        FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node: RecipeNodeId(2),
            body: 0,
        }
    }

    fn unresolved() -> FaceFrameFault {
        FaceFrameFault::Unresolved {
            error: InterrogateError::NoSuchName,
        }
    }

    /// **A face-frame fault about the pick is loud**, a stale latched
    /// pick included; one about a seat not yet answerable is quiet.
    #[test]
    fn a_face_frame_fault_takes_the_voice_the_fault_gives_it() {
        let (painted, voices) = landed_voiced(|ui| {
            let theme = &Theme::DEFAULT;
            face_frame_fault(
                ui,
                theme,
                &FaceFrameFault::NotPlanar {
                    carrier: SurfaceKind::Cylinder,
                },
                false,
            );
            face_frame_fault(
                ui,
                theme,
                &FaceFrameFault::NotOneBody {
                    at: RecipeNodeId(4),
                },
                false,
            );
            face_frame_fault(ui, theme, &unresolved(), false);
            face_frame_fault(ui, theme, &FaceFrameFault::NotLanded, false);
        });
        assert_eq!(
            find_opening(&painted, "a sketch frame is read off a PLANAR face").ink,
            Some(voices.unresolved)
        );
        assert_eq!(
            find_opening(&painted, "feature 4's value is several bodies").ink,
            Some(voices.unresolved)
        );
        assert_eq!(
            find_opening(&painted, "that face does not resolve: ").ink,
            Some(voices.unresolved)
        );
        assert_eq!(
            find_opening(&painted, "the document has not evaluated yet").ink,
            Some(voices.weak)
        );
    }

    /// **A stale pick the selection is already calling gone is not said
    /// a second time** by the form; one the selection is not about is.
    #[test]
    fn a_stale_latched_face_is_said_once_in_the_pane() {
        let (said_once, _) = landed_voiced(|ui| {
            face_frame_fault(ui, &Theme::DEFAULT, &unresolved(), true);
        });
        assert!(
            said_once.is_empty(),
            "{:?}",
            said_once.iter().map(|l| &l.text).collect::<Vec<_>>()
        );
        // Only `Unresolved` is the selection's to say.
        let (planar, _) = landed_voiced(|ui| {
            face_frame_fault(
                ui,
                &Theme::DEFAULT,
                &FaceFrameFault::NotPlanar {
                    carrier: SurfaceKind::Cylinder,
                },
                true,
            );
        });
        assert_eq!(planar.len(), 1);
    }

    /// **The selection says a latched face does not resolve exactly when
    /// it IS that face and no longer denotes** — the one case the form
    /// leaves the sentence to it.
    #[test]
    fn the_selection_speaks_for_the_latched_face_only_when_it_is_that_face_and_gone() {
        let face = |resolution: Option<Resolution>| Standing::Face {
            face: latched(),
            resolution: resolution.map(Box::new),
        };
        let gone = face(Some(Resolution::Failed(pncad::select::ResolutionFailure {
            error: pncad::select::ResolveError::NodeGone {
                name: latched().name,
                edit: editor_core::RecipeEditRef::NodeDeleted {
                    node: RecipeNodeId(1),
                },
            },
            offers: Vec::new(),
        })));
        assert!(selection_says_unresolved(&gone, Some(&latched())));
        // Another face, or nothing latched: the form must say it.
        let other = FaceSelection {
            node: RecipeNodeId(9),
            ..latched()
        };
        assert!(!selection_says_unresolved(&gone, Some(&other)));
        assert!(!selection_says_unresolved(&gone, None));
        // The latched face selected with no evaluation behind it: the
        // selection says "no evaluation yet", not that it is gone.
        assert!(!selection_says_unresolved(&face(None), Some(&latched())));
        // A node selected: the header is about the node.
        assert!(!selection_says_unresolved(
            &Standing::Node {
                node: RecipeNodeId(2),
                present: false,
            },
            Some(&latched())
        ));
    }

    /// **A bore at least as wide as the radius is a refused input**, and
    /// loud — not the form asking for its next one.
    #[test]
    fn a_bore_as_wide_as_the_radius_is_refused() {
        assert!(matches!(
            bore_held(true, 0.01, 0.01),
            Some(Held::Refused(_))
        ));
        assert_eq!(bore_held(true, 0.005, 0.01), None);
        assert_eq!(bore_held(false, 0.02, 0.01), None);
    }

    /// **The add-profile form's held reason**: a refused input is loud,
    /// the form waiting for one is quiet.
    #[test]
    fn a_held_profile_button_says_why_in_the_reasons_own_voice() {
        let (painted, voices) = landed_voiced(|ui| {
            held_line(ui, &Theme::DEFAULT, Held::Refused("the bore is too wide"));
            held_line(ui, &Theme::DEFAULT, Held::Waiting("choose a shape to add"));
        });
        assert_eq!(
            find(&painted, "the bore is too wide").ink,
            Some(voices.unresolved)
        );
        assert_eq!(
            find(&painted, "choose a shape to add").ink,
            Some(voices.weak)
        );
    }
}
