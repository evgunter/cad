//! The creation forms: the modal tools and the add-a-node panels the
//! Properties pane hosts above the selection.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{
    Coord, Doc, DocumentId, ProfileProgram, Said, Speaker, TracePassage, TraceSense, VarKind,
};
use pncad::select::SplitHalf;
use pncad::topo::BooleanOp;

use crate::app::ViewerBehavior;
use crate::blend::{BlendKindChoice, BlendTarget, FREEZE_NOTE};
use crate::combine::{BooleanTool, DUPLICATE_GAP, PatternOutputChoice, STEP_DIRECTION};
use crate::drafts::{CommitFault, Drafts, scalars};
use crate::forms::{
    ANGLE_DRAG_SPEED, COUNT_DRAG_SPEED, FIELD_DRAG_SPEED, PatternLayoutChoice, ShapeKind,
    UNIT_DRAG_SPEED, boolean_op_label, split_half_label,
};
use crate::frame::{self, Tone};
use crate::matetool::{MATE_KINDS, MateChoice, MateFreedom, MateTool};
use crate::pane::profile::{notation_row, path_steps_ui, preview_verdict};
use crate::parts::{PartChooser, PartEntry};
use crate::props::{Notation, render_number};
use crate::seats::{Seat, Seats, seat_line};
use crate::session::{Creation, NodeKindWanted, Selection, SessionOp, Staged, admits};
use crate::theme::Theme;
use crate::tools::{FormKind, ToolKind, ToolNotice};
use crate::tree::{self, PickedName};
use crate::widgets::{
    CANCEL, angle_picker, commit_row, door, form_door, length_picker, number_field, point_fields,
    unit_field, unit_vec3_row, vec3_row,
};

/// **The smallest instance index the part form offers.**
///
/// A family's members are indexed from zero, and an index below it
/// refuses at evaluation (`MemberOutOfRange`, typed, on the node's own
/// badge), so the form declines to author one — the same rule
/// [`MIN_PATTERN_COUNT`] follows. There is deliberately no upper
/// bound: how many instances the pattern has is a fact about its
/// VALUE, not about the document, so a cap here would be a limit read
/// off a picture that can change under it.
pub(crate) const MIN_PART_INSTANCE: i64 = 0;

/// **What a projection does to the picture**, said on the panel
/// before the click: the projected body is placed in the world, so it
/// is drawn beside whatever is drawn already (A10).
///
/// The pattern it reads is never drawn itself — it defines several
/// bodies, which no world placement reads — so it is reached from the
/// feature tree, and a person who wants another of its copies projects
/// that one too.
pub(crate) const PROJECTION_IS_PLACED: &str = "the selected copy is placed in the world and drawn: the pattern it is read out of is not \
     drawn itself — to project another of its copies, pick it again in the feature tree";

/// **The part form's rows**: the instance index, and what committing it
/// does to the picture ([`PROJECTION_IS_PLACED`]).
///
/// A free function over the `Ui`: `ViewerBehavior` borrows the whole
/// application, so this is the only seam a headless row can drive, and
/// the method's job is to call it.
pub(crate) fn part_selector_rows(ui: &mut egui::Ui, theme: &Theme, instance: &mut i64) {
    ui.horizontal(|ui| {
        ui.label("instance");
        ui.add(number_field(instance, COUNT_DRAG_SPEED).range(MIN_PART_INSTANCE..=i64::MAX));
    });
    crate::widgets::message_toned(
        ui,
        "instances are numbered from zero, in placement order",
        theme,
        Tone::Advisory,
    );
    crate::widgets::message_toned(ui, PROJECTION_IS_PLACED, theme, Tone::Advisory);
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
        "the copy lands beside the original along its own ({x}, {y}, {z}), clear of it by at \
         least {}% of the body's width — more for a body thin that way — measured off the body as it now is; \
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
///
/// A seat holding a split picked in the tree reads one of its halves,
/// still to choose: the row offers the two, and answers the seat and
/// the half a click chose, which the caller hands to the open tool
/// ([`crate::tools::Tools::choose_half`]).
pub(crate) fn seats_row(
    ui: &mut egui::Ui,
    seats: &Seats,
    doc: &Doc<ProfileProgram>,
    theme: &Theme,
) -> Option<(usize, SplitHalf)> {
    crate::widgets::message_toned(ui, seat_line(seats, doc), theme, Tone::Advisory);
    half_rows(
        ui,
        (0..2)
            .filter(|&i| seats.awaits_half(i))
            .map(|i| (i, seats.role(i).name().to_owned())),
        theme,
    )
}

/// **One row per pick still waiting for its half**, each offering the
/// two: `waiting` names the pick's index and how the panel calls it,
/// and the answer is the pick and the half a click chose. Under the
/// rows, which side is which.
pub(crate) fn half_rows(
    ui: &mut egui::Ui,
    waiting: impl IntoIterator<Item = (usize, String)>,
    theme: &Theme,
) -> Option<(usize, SplitHalf)> {
    let mut chosen = None;
    let mut any = false;
    for (i, said) in waiting {
        any = true;
        ui.horizontal(|ui| {
            ui.label(format!("{said}: read the"));
            // One button per half the KERNEL has, in its order: the row
            // offers the vocabulary, never a copy of it.
            for half in SplitHalf::ALL {
                if ui.button(split_half_label(half)).clicked() {
                    chosen = Some((i, half));
                }
            }
            ui.label("half");
        });
    }
    if any {
        crate::widgets::message_toned(
            ui,
            "the split plane's normal side is above",
            theme,
            Tone::Advisory,
        );
    }
    chosen
}

/// **The boolean tool's picks still waiting for a half**, each called
/// as its panel line calls it: a member by its place in the list, a
/// subtraction's seat by its role.
pub(crate) fn boolean_waiting(tool: &BooleanTool) -> Vec<(usize, String)> {
    tool.awaiting_half()
        .into_iter()
        .map(|i| {
            let said = match tool.operation() {
                BooleanOp::Subtract => [Seat::SubtractFrom, Seat::SubtractTool][i]
                    .name()
                    .to_owned(),
                BooleanOp::Union | BooleanOp::Intersect => {
                    format!("{} {}", Seat::Member.name(), i + 1)
                }
            };
            (i, said)
        })
        .collect()
}

/// **What the boolean tool asks for under `op`**: any number of bodies
/// for union and intersect, the two seats in order for subtract.
pub(crate) fn boolean_prompt(op: BooleanOp) -> &'static str {
    match op {
        BooleanOp::Union | BooleanOp::Intersect => "pick the bodies, one or more, then commit",
        BooleanOp::Subtract => "pick the body to keep, then the body to remove",
    }
}

/// **The mate tool's held picks, drawn** — [`MateTool::line`], the
/// seated tools' line over the mate's two sides, in the same voice as
/// [`seats_row`].
pub(crate) fn mate_picks_row(ui: &mut egui::Ui, tool: &MateTool, by: Speaker<'_>, theme: &Theme) {
    crate::widgets::message_toned(ui, tool.line(by), theme, Tone::Advisory);
}

/// **The smallest pattern count the form offers.**
///
/// A pattern of zero instances refuses at evaluation
/// (`CountOutOfRange`, typed, on the node's own badge), so the form
/// declines to author one — the same rule the bore field follows. There
/// is deliberately NO upper bound: the property panel imposes none on
/// the slot afterwards, and a cap here would be a limit the document
/// does not have.
pub(crate) const MIN_PATTERN_COUNT: i64 = 1;

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

/// **One part the chooser offers**: its pick button and file name on
/// one line, and the document's id on a line of its own under it.
/// Answers whether the button was clicked.
///
/// The file name and the id are NAMES, not sentences, so neither
/// wraps: each is drawn whole or, in a window narrower than it, elided
/// with the whole text on hover — egui's own truncation. The file name
/// is the user's and nothing here bounds it, which is why it is not
/// the button's label: a button lays its text out on one line at any
/// width, and the window would take that width. The file name is what
/// a reader picks by (`crate::parts::catalogue` sorts on it); the id
/// tells two same-named files apart.
///
/// An entry that cannot be picked stays VISIBLE and disabled, carrying
/// the op's own refusal — read off the entry, not minted here.
fn part_entry(ui: &mut egui::Ui, theme: &Theme, entry: &PartEntry) -> bool {
    let refusal = entry.refusal();
    let picked = ui
        .horizontal(|ui| {
            let picked = crate::app::refusable_button(ui, PICK_PART, refusal.as_ref());
            ui.add(egui::Label::new(entry.file_name()).truncate());
            picked
        })
        .inner;
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

/// What [`part_entry`]'s pick button says.
const PICK_PART: &str = "add";

/// What the button that opens the part chooser says.
pub(crate) const ADD_PART: &str = "Add part…";

/// What the extrude form's commit button says, before the profile's
/// number once one is selected.
pub(crate) const EXTRUDE: &str = "Extrude";

/// What the add-profile form's commit button says.
pub(crate) const ADD_PROFILE: &str = "Add profile";

/// Why the add-profile form's commit waits while its preview refuses
/// the loops: the refusal itself is said above it.
const PREVIEW_REFUSES: &str = "the preview above refuses these loops";

/// What the door that opens the add-profile form says.
pub(crate) const ADD_PROFILE_DOOR: &str = "Add profile…";

/// What the add-profile form asks for while it draws on a face and
/// holds none.
pub(crate) const PICK_A_FACE: &str = "pick a face in the viewport to draw on";

/// The traced path's latch for the plane face the sketch is drawn in:
/// square to the traced face's axis.
pub(crate) const TRACE_PLANE: &str = "sketch plane on";

/// The traced path's latch for the face it traces.
pub(crate) const TRACE_FACE: &str = "trace";

/// What the add-profile form says while no frame reads off the traced
/// path's picks.
pub(crate) const NO_TRACED_FRAME: &str =
    "pick a traced face with an axis and a plane face square to it";

/// The highest crossing the traced path's row offers: a line meets a
/// quadric's trace twice and a torus's four times, so a count past this
/// names a crossing no piece makes.
const MAX_TRACE_CROSSING: u32 = 4;

/// What a traced path's latch button says: it takes the viewport's face
/// pick.
pub(crate) const USE_THE_PICKED_FACE: &str = "use the picked face";

/// What the split panel says of the plane's numbers: which coordinates
/// they are in.
const SPLIT_COORDINATES: &str = "pick the body; the plane's point and normal are in the coordinates \
     of the space the body is built in, and the viewport draws the plane on the copy you picked";

/// The split panel's line for a target picked without the viewport (a
/// tree click): no drawn part was hit, so the viewport has nowhere to
/// draw the plane.
const SPLIT_NOT_AIMED: &str = "the plane is not drawn: the body was not picked in the viewport; pick it there to see the plane on it";

/// The split panel's line for a viewport pick whose part the picture no
/// longer draws (hidden, or gone from the picture), or whose body has no
/// landed value to draw the plane against.
const SPLIT_NOT_DRAWN: &str =
    "the plane is not drawn: what was picked is not in the picture as it stands";

/// The split panel's line for a target with no corner to centre the
/// plane's point on.
const SPLIT_NO_CENTRE: &str = "the body has no corners to centre the plane on, so the plane starts through its space's origin";

/// The split panel's line while the staged split runs.
const SPLIT_CHECKING: &str = "checking the plane against the body…";

/// The split panel's line for a plane with material on both sides.
const SPLIT_CUTS: &str = "the plane cuts the body in two";

/// The split panel's verdict on a plane that leaves a side empty, before
/// the row's own word for which.
const SPLIT_LEAVES: &str = "committed as it stands, the split would read";

/// Why a side is empty, said beside [`SPLIT_LEAVES`].
const SPLIT_MISSES: &str = "the plane misses the body or lies on one of its faces";

/// The kind nouns of the nodes the forms with no [`ToolKind`] create —
/// `node_kind_noun`'s words, which a form's proposed label counts by.
/// `creation_nouns` (below) holds the profile's, the extrude's and the
/// part chooser's to the node their op mints.
const PROFILE_NOUN: &str = "Profile";
const EXTRUDE_NOUN: &str = "Extrude";

/// What the mate tool writes for a freedom left unticked: every bundle
/// pins its copy, so an unset freedom is written where the copy stands,
/// snapped onto the mate, never left free.
pub(crate) const MATE_UNSET_SNAPS: &str = "Every bundle pins its copy: a freedom left unticked is written where the copy stands, \
     snapped onto the mate.";
pub(crate) const INSTANCE_NOUN: &str = "InstantiatePart";

/// The heading of the section that makes a body out of nothing.
pub(crate) const ADD_FEATURE: &str = "Add feature";

/// The heading of the section whose tools take bodies that exist and
/// make another one.
pub(crate) const COMBINE_BODIES: &str = "Combine bodies";

/// The heading of the section whose tools reshape a body's edges.
pub(crate) const BLEND_EDGES: &str = "Blend edges";

/// The heading of the section whose tools bring in another document
/// and place it.
pub(crate) const ASSEMBLE_PARTS: &str = "Assemble parts";

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

/// **Why the add-profile button is withheld**, when it is — and how loud
/// that is, which depends on which of two things it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Withheld {
    /// The form is waiting for its next input: a face, a shape, a
    /// first step. The form asking, [`Tone::Advisory`].
    Waiting(&'static str),
    /// An input the reader gave is refused, and the button stays shut
    /// until they change it: [`Tone::Actionable`].
    Refused(&'static str),
}

impl Withheld {
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

/// **The one reason the add-profile button is withheld for**, out of every
/// reason the form found, handed over in the form's own top-to-bottom
/// order.
///
/// A refused input outranks the form waiting for one: a missing face,
/// shape or first step shows in the form itself (no face held, no
/// shape chosen, an empty chain), while a refused value looks like any
/// other number in its field and has no other voice. Between two of
/// one kind the earlier in the form is said, so the first thing a
/// person is asked for is the first thing the form lacks.
fn withheld_for(holds: impl IntoIterator<Item = Withheld>) -> Option<Withheld> {
    holds.into_iter().reduce(|said, next| match (said, next) {
        (Withheld::Waiting(_), Withheld::Refused(_)) => next,
        _ => said,
    })
}

/// **What a bored circle withholds the button for**: a bore at least as
/// wide as the radius, which is an input the reader gave and the form
/// refuses — [`Withheld::Refused`], not a request for the next input.
fn bore_withholds(bored: bool, bore: f64, radius: f64) -> Option<Withheld> {
    (bored && bore >= radius).then_some(Withheld::Refused(
        "the bore must be smaller than the radius — which loop is the hole is decided by \
         containment, so a larger bore would swap the roles rather than refuse",
    ))
}

/// The withheld button's reason, drawn in its own voice. A free function
/// over the `Ui` so a headless drive can reach it.
fn withheld_line(ui: &mut egui::Ui, theme: &Theme, withheld: Withheld) {
    crate::widgets::message_toned(ui, withheld.words(), theme, withheld.tone());
}

impl ViewerBehavior<'_> {
    /// The creation sections, each behind its own header and each tool
    /// or form in it behind its own button, so the pane at rest is the
    /// headers. A form whose op takes the selection is not here: it is
    /// drawn beside the selection while its op admits it
    /// ([`Self::extrude_ui`]). Each form is minimal — its few required
    /// fields with sensible defaults — and emits exactly one creation
    /// op; the property panel is the editor for everything after the
    /// insert — for a profile, through this section's own path editor
    /// opened on the node ([`Self::edit_profile_ui`]).
    pub(crate) fn create_ui(&mut self, ui: &mut egui::Ui) {
        ui.collapsing(ADD_FEATURE, |ui| {
            self.add_profile_ui(ui);
            ui.separator();
            self.revolve_tool_ui(ui);
        });
        // The combining tools sit in their own section:
        // everything above makes a body out of nothing, everything
        // here takes bodies that exist and makes another one.
        ui.collapsing(COMBINE_BODIES, |ui| {
            self.boolean_tool_ui(ui);
            ui.separator();
            self.split_tool_ui(ui);
            ui.separator();
            self.mirror_tool_ui(ui);
            ui.separator();
            self.transform_tool_ui(ui);
            ui.separator();
            self.pattern_tool_ui(ui);
            ui.separator();
            // Beside the pattern they read and author: a projection
            // takes ONE body out of a split's or a pattern's several,
            // and a duplicate authors one stepped copy of a body.
            self.projection_tool_ui(ui);
            ui.separator();
            self.duplicate_tool_ui(ui);
        });
        // The blend tools sit in their own section: they
        // take a body that exists and reshape its EDGES, which is a
        // third kind of move again — and the only one whose picks are
        // a set rather than a seat.
        ui.collapsing(BLEND_EDGES, |ui| {
            self.blend_tool_ui(ui);
        });
        // The tools whose subject is another document: an instance of
        // a part, and the mate that places one.
        ui.collapsing(ASSEMBLE_PARTS, |ui| {
            self.add_part_ui(ui);
            self.mate_tool_ui(ui);
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
            return self.tool_door(ui, ToolKind::Mate);
        };
        crate::widgets::message(ui, ToolKind::Mate.says(&"pick two faces in the viewport"));
        mate_picks_row(
            ui,
            &tool,
            frame::landed_speaker(self.session.landed_pair()),
            &self.theme,
        );
        ui.horizontal(|ui| {
            for (ix, (_, label)) in MATE_KINDS.iter().enumerate() {
                ui.radio_value(&mut self.drafts.mate_kind, ix, *label);
            }
        });
        let kind = MATE_KINDS
            .get(self.drafts.mate_kind)
            .map_or(VarKind::Plane, |(kind, _)| *kind);
        ui.checkbox(&mut self.drafts.mate_flip, "faces rest against each other");
        ui.checkbox(
            &mut self.drafts.mate_into_target_space,
            "move the copy into the target's space",
        );
        // The freedoms this kind leaves, each set to its number when
        // ticked. Every bundle pins its copy, so a freedom left unticked
        // is written as the constant 0 (the bundle door's rule), and the
        // panel says so rather than leaving it to be found out.
        ui.label(MATE_UNSET_SNAPS);
        for (freedom, (set, value)) in MateFreedom::ALL.iter().zip(&mut self.drafts.mate_values) {
            let coord = freedom.coord();
            if !MateFreedom::offered_beside(kind) {
                continue;
            }
            ui.horizontal(|ui| {
                ui.checkbox(set, coord.name());
                let (unit, speed) = match coord {
                    Coord::Slide(_) => (self.notation.length.def(), FIELD_DRAG_SPEED),
                    Coord::Spin(_) => (self.notation.angle.def(), ANGLE_DRAG_SPEED),
                };
                unit_field(ui, unit, speed, value);
            });
        }
        let closing = commit_row(ui, ToolKind::Mate.commit(), None, |_| {});
        let mut close = closing.cancel;
        if closing.commit {
            let values: Result<Vec<_>, _> = MateFreedom::ALL
                .iter()
                .zip(self.drafts.mate_values)
                .filter(|(_, (set, _))| *set && MateFreedom::offered_beside(kind))
                .map(|(freedom, (_, value))| {
                    let coord = freedom.coord();
                    match coord {
                        Coord::Slide(_) => self.notation.length_literal(value),
                        Coord::Spin(_) => self.notation.angle_literal(value),
                    }
                    .map(|by| (coord, by))
                })
                .collect();
            match (values, self.session.landed_pair()) {
                (Err(error), _) => {
                    self.notices
                        .push(frame::tool_news(&ToolKind::Mate.refuses(&error)));
                }
                (Ok(values), Some((doc, _))) => {
                    let choice = MateChoice {
                        kind,
                        flip: self.drafts.mate_flip,
                        values,
                    };
                    match tool.proposal(doc, choice) {
                        Ok(proposal) => {
                            // One committed action; the tool
                            // closes with it.
                            let op = if self.drafts.mate_into_target_space {
                                proposal.op_into_target_space()
                            } else {
                                proposal.op()
                            };
                            self.push_tool_op(self.tool_noun(ToolKind::Mate), op);
                            close = true;
                        }
                        // Said from the document the panel's
                        // line speaks, so a rename that has
                        // not landed reads the same in both.
                        Err(error) => {
                            self.notices.push(frame::tool_news(
                                &ToolKind::Mate.refuses(&error.respoken(self.session.doc())),
                            ));
                        }
                    }
                }
                _ => {
                    self.notices.push(frame::tool_news(
                        &ToolKind::Mate.refuses(&"no landed document to read the picks on"),
                    ));
                }
            }
        }
        if close {
            self.tools.close();
        }
        ui.separator();
    }

    /// The `Add part…` door: the open document's own directory,
    /// listed as parts, one click inserting an instance of one. The
    /// label field stands above the listing because a pick commits at
    /// once, so the label is typed first.
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
            if door(ui, ADD_PART)
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
        // Taken out only so the label row can borrow `self` while the
        // window draws: the chooser stays open, and no early return may
        // sit between this take and the put-back below.
        if let Some(chooser) = self.part_chooser.take() {
            part_window(ui).show(ui.ctx(), |ui| {
                self.creation_label_row(ui, INSTANCE_NOUN);
                chosen = part_listing(ui, &self.theme, &chooser);
                ui.horizontal(|ui| {
                    if ui
                        .button("Rescan")
                        .on_hover_text("re-read this directory")
                        .clicked()
                    {
                        rescan = true;
                    }
                    if ui.button(CANCEL).clicked() {
                        close = true;
                    }
                });
            });
            // The put-back the take above owes.
            *self.part_chooser = Some(chooser);
        }
        if let Some(id) = chosen {
            // Exactly one committed edit (and its label), and the
            // chooser closes with it — the mate tool's shape.
            self.push_labelled(INSTANCE_NOUN, SessionOp::AddInstance { id });
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

    /// The add-profile form: a template shape with Length fields, one
    /// [`SessionOp::AddProfile`] on commit — the profile on its own
    /// plane — or, drawing on a face, one [`SessionOp::SketchOnFace`]:
    /// the profile extruded away from the face picked in the viewport
    /// and fused with its body, in one submit and one undo. Behind its
    /// `Add profile…` door, as every tool is behind its button: at rest
    /// the form is one control, and it closes with its commit
    /// ([`crate::drafts::Drafts::accepted`]) or its Cancel.
    ///
    /// The circle's optional bore is what lets this template author
    /// the hollow ring's annulus (one profile node, two loops).
    ///
    /// The bore field is guarded IN THE FORM: loop roles come from
    /// the profile layer's containment forest, not from list order,
    /// so a bore at or beyond the outer radius would not refuse — it
    /// would silently swap which circle is the hole. The form says
    /// "bore", so it disables its commit until the bore is smaller, with
    /// the reason shown. This is a chrome affordance guarding the
    /// template's stated intent; the op stays unjudged and the
    /// kernel's containment rule stays the one home.
    pub(crate) fn add_profile_ui(&mut self, ui: &mut egui::Ui) {
        if !form_door(ui, ADD_PROFILE_DOOR, &mut self.drafts.profile_open) {
            return;
        }
        self.profile_drawn.create = true;
        ui.horizontal(|ui| {
            ui.label("profile");
            for (shape, label) in ShapeKind::ALL {
                ui.radio_value(&mut self.drafts.profile_shape, Some(shape), label);
            }
        });
        ui.checkbox(&mut self.drafts.sketch_on_face, "on the picked face");
        if self.drafts.sketch_on_face {
            self.sketch_face_rows(ui);
        }
        let shape = self.drafts.profile_shape;
        // Every reason the button is withheld, in the form's order; which
        // one is said is `withheld_for`'s to decide, not any one arm's.
        let mut withholds: Vec<Withheld> = Vec::new();
        if self.drafts.sketch_on_face && self.drafts.held_face().is_none() {
            withholds.push(Withheld::Waiting(PICK_A_FACE));
        }
        match shape {
            // No shape chosen: the form is at rest. It says what it is
            // waiting for and draws nothing — no fields to fill in for
            // a shape nobody picked, and no preview in the viewport.
            None => withholds.push(Withheld::Waiting("choose a shape to add")),
            Some(ShapeKind::Circle) => {
                let unit = self.notation.length.def();
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
                    length_picker(ui, "profile_circle", &mut self.notation.length);
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.drafts.profile_bored, "with bore");
                    if self.drafts.profile_bored {
                        ui.label("bore radius");
                        unit_field(ui, unit, FIELD_DRAG_SPEED, &mut self.drafts.profile_bore);
                    }
                });
                withholds.extend(bore_withholds(
                    self.drafts.profile_bored,
                    self.drafts.profile_bore,
                    self.drafts.profile_radius,
                ));
            }
            Some(ShapeKind::Rectangle) => {
                let unit = self.notation.length.def();
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
                    length_picker(ui, "profile_rectangle", &mut self.notation.length);
                });
            }
            Some(ShapeKind::Path) => {
                notation_row(
                    ui,
                    "path",
                    &mut self.notation.length,
                    &mut self.notation.angle,
                );
                if let Some(row) = path_steps_ui(
                    ui,
                    "path",
                    self.session.tol(),
                    &mut crate::widgets::WrittenIn::of(*self.notation),
                    &mut self.drafts.profile_path,
                ) {
                    row.apply(&mut self.drafts.profile_path);
                }
                // A chain with no steps is a form waiting for its
                // first one, not a chain that fails to close. Without
                // this the empty list drew the lattice's own refusal
                // about a program nobody had started writing.
                if self.drafts.profile_path.is_empty() {
                    withholds.push(Withheld::Waiting("add a step to the chain"));
                }
                if !self.drafts.sketch_on_face {
                    ui.checkbox(
                        &mut self.drafts.trace_on,
                        "continue tangent to a face's trace",
                    );
                }
                if self.drafts.traces() {
                    withholds.extend(self.trace_rows(ui));
                }
            }
        }
        let blocked = withheld_for(withholds);
        if let Some(withheld) = blocked {
            withheld_line(ui, &self.theme, withheld);
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
        // A traced chain is drawn by the evaluation, which the preview
        // does not run: the trace rows say what it shows instead.
        let at_rest = shape.is_none()
            || self.drafts.profile_path.is_empty() && blocked.is_some()
            || self.drafts.traces();
        let refused = preview_verdict(
            ui,
            self.theme,
            self.profile_previews.create.as_ref().filter(|_| !at_rest),
        );
        self.creation_label_row(ui, PROFILE_NOUN);
        let withheld = match blocked {
            Some(withheld) => Some(withheld.words()),
            None => refused.then_some(PREVIEW_REFUSES),
        };
        let closing = commit_row(ui, ADD_PROFILE, withheld, |_| {});
        if closing.cancel {
            self.drafts.profile_open = false;
        }
        if closing.commit {
            // Lowered HERE rather than in the session: the notation is
            // the form's, and a literal that forgot it between the two
            // is exactly the gap this carries across.
            match self.drafts.profile_op(*self.notation) {
                Ok(Some(op)) => self.push_labelled(PROFILE_NOUN, op),
                // Unreachable while the button is gated on `blocked`,
                // and typed rather than unwrapped: a form's enabling
                // condition and its commit are two pieces of code, and
                // this one does not assume the other got it right.
                Ok(None) => {
                    self.notices.push(frame::tool_news(
                        &FormKind::AddProfile.refuses(&PICK_A_FACE),
                    ));
                }
                Err(error) => {
                    self.notices
                        .push(frame::tool_news(&FormKind::AddProfile.refuses(&error)));
                }
            }
        }
    }

    /// **The traced path's rows** ([`SessionOp::AddDrawnProfile`]): the
    /// two latches — the face whose body's plane the sketch is drawn on,
    /// and the face it traces — each taking the viewport's face pick on
    /// its button; the trace the two show, or the trace's own refusal;
    /// and the gesture's row and branches. Answers what the commit still
    /// waits for.
    fn trace_rows(&mut self, ui: &mut egui::Ui) -> Vec<Withheld> {
        let picked = match self.session.selection() {
            Selection::Face(face) => Some(face.clone()),
            _ => None,
        };
        let speaker = frame::landed_speaker(self.session.landed_pair());
        let mut withholds = Vec::new();
        for (label, plane) in [(TRACE_PLANE, true), (TRACE_FACE, false)] {
            ui.horizontal(|ui| {
                ui.label(label);
                let held = if plane {
                    &mut self.drafts.trace_plane
                } else {
                    &mut self.drafts.trace_face
                };
                match held.as_ref() {
                    Some(face) => ui.weak(Said(&PickedName::face(face), speaker).to_string()),
                    None => ui.weak("none picked"),
                };
                if ui
                    .add_enabled(picked.is_some(), egui::Button::new(USE_THE_PICKED_FACE))
                    .clicked()
                {
                    held.clone_from(&picked);
                }
            });
        }
        if self.drafts.trace_plane.is_none() {
            withholds.push(Withheld::Waiting(
                "pick the face whose plane the sketch is on",
            ));
        }
        if self.drafts.trace_face.is_none() {
            withholds.push(Withheld::Waiting("pick the face to trace"));
        }
        match self.drafts.trace_shown.as_ref().map(|shown| &shown.read) {
            Some(Ok(marks)) => {
                crate::widgets::message(
                    ui,
                    format!(
                        "the trace: {} points drawn, {} rays refused",
                        marks.marks.len(),
                        marks.refused
                    ),
                );
            }
            Some(Err(fault)) => {
                crate::widgets::message_toned(
                    ui,
                    format!("the trace: {fault}"),
                    &self.theme,
                    Tone::Actionable,
                );
                // A frame that does not read is refused before Add: the
                // insert would refuse it in the same words.
                if matches!(fault, crate::session::SketchTraceFault::Frame(_)) {
                    withholds.push(Withheld::Refused(NO_TRACED_FRAME));
                }
            }
            None => {}
        }
        let rows = self.drafts.profile_path.len().max(1);
        let gesture = &mut self.drafts.trace_gesture;
        ui.horizontal(|ui| {
            ui.label("after row");
            let mut row = gesture.after + 1;
            ui.add(number_field(&mut row, COUNT_DRAG_SPEED).range(1..=rows));
            gesture.after = row.saturating_sub(1);
        });
        ui.horizontal(|ui| {
            ui.label("to crossing");
            ui.add(
                number_field(&mut gesture.crossing.nth, COUNT_DRAG_SPEED)
                    .range(1..=MAX_TRACE_CROSSING),
            );
            for passage in [TracePassage::Leave, TracePassage::Enter] {
                ui.radio_value(&mut gesture.crossing.passage, passage, passage.word());
            }
        });
        ui.horizontal(|ui| {
            ui.label("then along the trace");
            for (sense, word) in [(TraceSense::With, "with"), (TraceSense::Against, "against")] {
                ui.radio_value(&mut gesture.sense, sense, word);
            }
        });
        crate::widgets::message(
            ui,
            "the piece after that row runs to the crossing, counted along it from the pen, and \
             the chain heads along the trace there: both are read off the traced face at every \
             evaluation, and a crossing the piece does not make refuses on the profile",
        );
        withholds
    }

    /// **The on-face rows**: the held face pick, the depth the sketch is
    /// extruded away from it, and its spin.
    ///
    /// The seat LATCHES the viewport's face pick — a face is not a
    /// node a combo can list, so the selection is this form's picker
    /// and there is no widget to draw for it. Writing it here rather
    /// than reading the live selection at the commit is what lets an
    /// author pick a face, type a depth and click the feature tree
    /// without losing the pick; a second face pick moves the seat.
    fn sketch_face_rows(&mut self, ui: &mut egui::Ui) {
        if let Selection::Face(face) = self.session.selection() {
            self.drafts
                .hold_sketch_face(face.clone(), self.session.selection_said());
        }
        ui.horizontal(|ui| {
            ui.label("face");
            match self.drafts.held_face() {
                // Said from the landed pair the pick was read off, a
                // node that document no longer holds as the form kept it.
                Some(face) => {
                    let by = frame::landed_speaker(self.session.landed_pair())
                        .or_held(self.drafts.sketch_face_said());
                    ui.weak(Said(&PickedName::face(face), by).to_string())
                }
                None => ui.weak("none picked"),
            };
        });
        ui.horizontal(|ui| {
            ui.label("depth");
            unit_field(
                ui,
                self.notation.length.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.sketch_depth,
            );
            length_picker(ui, "sketch_depth", &mut self.notation.length);
        });
        ui.horizontal(|ui| {
            ui.label("spin");
            unit_field(
                ui,
                self.notation.angle.def(),
                ANGLE_DRAG_SPEED,
                &mut self.drafts.sketch_spin,
            );
            angle_picker(ui, "sketch_spin", &mut self.notation.angle);
        });
        // What the numbers MEAN, said where they are typed.
        crate::widgets::message(
            ui,
            "extruded away from the face and fused with its body; its +x turned about the face's \
             outward normal",
        );
    }

    /// The extrude form: the current selection is the profile (a tree
    /// pick, or a face pick whose feature is one — `Selection::node`),
    /// one distance field, one [`SessionOp::AddExtrude`] on commit.
    ///
    /// **Drawn only while a profile is selected**, by the test the
    /// door itself refuses with ([`admits`] for
    /// [`NodeKindWanted::Profile`], over the committed document as
    /// `DocSession::require_kind` reads it): the form is offered
    /// exactly where the op would accept it, as the instance section
    /// is (`ViewerBehavior::instance_ui`) — and beside the selection,
    /// outside the collapsed creation sections ([`Self::create_ui`]),
    /// so no header has to be opened to find it.
    pub(crate) fn extrude_ui(&mut self, ui: &mut egui::Ui) {
        let Some(node) =
            self.session.selection().node().filter(|node| {
                admits(self.session.committed_doc(), *node, NodeKindWanted::Profile)
            })
        else {
            return;
        };
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("extrude");
            ui.label("distance");
            unit_field(
                ui,
                self.notation.length.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.extrude_distance,
            );
            length_picker(ui, "extrude_distance", &mut self.notation.length);
        });
        self.creation_label_row(ui, EXTRUDE_NOUN);
        if ui
            .button(format!(
                "{EXTRUDE} {}",
                Speaker::of(self.session.doc())
                    .or_held(self.session.selection_said())
                    .node(node)
            ))
            .clicked()
        {
            match self.notation.length_literal(self.drafts.extrude_distance) {
                Ok(distance) => self.push_labelled(
                    EXTRUDE_NOUN,
                    SessionOp::AddExtrude {
                        profile: node,
                        distance,
                    },
                ),
                Err(error) => {
                    self.notices
                        .push(frame::tool_news(&FormKind::Extrude.refuses(&error)));
                }
            }
        }
    }

    /// The revolve tool's panel: activation, the held profile pick, the
    /// axis line in the profile's own coordinates, the angle field, and
    /// the one committed edit — the same chrome shape as every other
    /// seated tool.
    pub(crate) fn revolve_tool_ui(&mut self, ui: &mut egui::Ui) {
        // A copy of the small tool value, for the reason the mate
        // panel takes one: the panel reads it while pushing ops and
        // closing the tool; the authoritative copy is only ever
        // replaced whole.
        let Some(tool) = self.tools.revolve() else {
            return self.tool_door(ui, ToolKind::Revolve);
        };
        crate::widgets::message(
            ui,
            ToolKind::Revolve.says(&"pick the profile, then write its axis"),
        );
        if let Some((seat, half)) = seats_row(ui, tool.seats(), self.session.doc(), &self.theme) {
            self.tools.choose_half(seat, half);
        }
        let unit = self.notation.length.def();
        // Whether a field below was edited: the viewport may already
        // have drawn this frame's axis from the old values, so an edit
        // asks for the next frame.
        let mut edited = false;
        ui.horizontal(|ui| {
            ui.label("axis through");
            edited |= point_fields(ui, unit, &mut self.drafts.revolve_axis_through);
            length_picker(ui, "revolve_axis_through", &mut self.notation.length);
        });
        ui.horizontal(|ui| {
            ui.label("axis heading");
            edited |= unit_field(
                ui,
                self.notation.angle.def(),
                ANGLE_DRAG_SPEED,
                &mut self.drafts.revolve_axis_heading,
            );
        });
        // Where the numbers are measured, said where they are typed.
        crate::widgets::message(
            ui,
            "x, y and the heading from x are the profile's own; the axis lies in its plane",
        );
        crate::widgets::message(
            ui,
            "the viewport draws the axis: the profile must lie on the side its ticks point to, \
             and a positive angle turns it right-handed about the arrow",
        );
        ui.horizontal(|ui| {
            ui.label("angle");
            edited |= unit_field(
                ui,
                self.notation.angle.def(),
                ANGLE_DRAG_SPEED,
                &mut self.drafts.revolve_angle,
            );
            angle_picker(ui, "revolve_angle", &mut self.notation.angle);
        });
        if edited {
            ui.ctx().request_repaint();
        }
        self.tool_commit_row(ui, ToolKind::Revolve, |drafts, notation| {
            Ok(tool.op(
                notation.point_literals(drafts.revolve_axis_through.to_array())?,
                notation.angle_literal(drafts.revolve_axis_heading)?,
                notation.angle_literal(drafts.revolve_angle)?,
            )?)
        });
    }

    /// The boolean tool's panel: activation, the operation choice, the
    /// held picks — a member list for union and intersect, the two seats
    /// named by ROLE for subtract — and the one committed edit.
    ///
    /// The role naming is the point of the subtract panel: it removes
    /// one pick from the other, so a user who cannot see which is which
    /// cannot author the operation they mean. A union or an intersect
    /// commits whatever members are held, one or more.
    pub(crate) fn boolean_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.boolean() else {
            return self.tool_door(ui, ToolKind::Boolean);
        };
        let mut op = tool.operation();
        ui.horizontal(|ui| {
            ui.label("operation");
            // One button per operation the KERNEL has, in its order:
            // the form offers the vocabulary, never a copy of it.
            for &choice in BooleanOp::ALL {
                ui.radio_value(&mut op, choice, boolean_op_label(choice));
            }
        });
        if op != tool.operation()
            && let Some(open) = self.tools.boolean_mut()
        {
            open.set_operation(op);
        }
        let Some(tool) = self.tools.boolean() else {
            return;
        };
        crate::widgets::message(
            ui,
            ToolKind::Boolean.says(&boolean_prompt(tool.operation())),
        );
        crate::widgets::message_toned(
            ui,
            tool.line(self.session.doc()),
            &self.theme,
            Tone::Advisory,
        );
        if let Some((pick, half)) = half_rows(ui, boolean_waiting(&tool), &self.theme) {
            self.tools.choose_half(pick, half);
        }
        let Some(tool) = self.tools.boolean() else {
            return;
        };
        if clear_picks_button(ui, tool.picks().len(), Picked::Bodies)
            && let Some(open) = self.tools.boolean_mut()
        {
            open.clear();
        }
        self.tool_commit_row(ui, ToolKind::Boolean, |_, _| Ok(tool.op()?));
    }

    /// The split tool's panel: a body pick, the cutting plane's point
    /// and normal in the body's own coordinates, and the one committed
    /// edit.
    pub(crate) fn split_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.split() else {
            return self.tool_door(ui, ToolKind::Split);
        };
        crate::widgets::message(ui, ToolKind::Split.says(&SPLIT_COORDINATES));
        if let Some((seat, half)) = seats_row(ui, tool.seats(), self.session.doc(), &self.theme) {
            self.tools.choose_half(seat, half);
        }
        self.seed_split_plane(&tool);
        let drawn = (self.drafts.split_origin, self.drafts.split_normal);
        ui.horizontal(|ui| {
            unit_vec3_row(
                ui,
                "plane through",
                self.notation.length.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.split_origin,
            );
            length_picker(ui, "split_origin", &mut self.notation.length);
        });
        vec3_row(ui, "normal", UNIT_DRAG_SPEED, &mut self.drafts.split_normal);
        // The viewport drew this frame's plane before the panel ran: an
        // edit asks for the next frame, which draws it as typed.
        if (self.drafts.split_origin, self.drafts.split_normal) != drawn {
            ui.ctx().request_repaint();
        }
        self.split_plane_rows(ui, &tool);
        self.tool_commit_row(ui, ToolKind::Split, |drafts, notation| {
            Ok(tool.op(
                notation.length_literals(drafts.split_origin)?,
                scalars(drafts.split_normal)?,
            )?)
        });
    }

    /// **The split plane's point, seeded for a new target**: at the
    /// centre of the body's extent in its own coordinates
    /// ([`crate::splitplane::centre`]), so the plane the panel opens
    /// with passes through the body rather than along one of its faces.
    /// Seeded once per target, so what the person types stands.
    fn seed_split_plane(&mut self, tool: &crate::combine::SplitTool) {
        let target = tool.seats().read(0);
        if target == self.drafts.split_seeded {
            return;
        }
        let Some(read) = target.as_ref() else {
            self.drafts.split_seeded = None;
            return;
        };
        let Some((doc, evaluation)) = self.session.landed_pair() else {
            return;
        };
        let Some(source) = crate::splitplane::source(doc, evaluation, read) else {
            return;
        };
        // A body with no corner has no centre to seed: the plane starts
        // at the origin, and the panel says why ([`SPLIT_NO_CENTRE`]).
        self.drafts.split_origin = crate::splitplane::centre(source).unwrap_or([0.0; 3]);
        self.drafts.split_seeded = target;
    }
    /// **What the split plane does to its target, before commit**, and
    /// whether the viewport draws it.
    ///
    /// The verdict is the split door's own: the split the panel would
    /// commit, staged on the committed document and evaluated beside
    /// the picture ([`SessionOp::Stage`]), read as its tree row would
    /// read it ([`tree::split_reading`]). It is asked again whenever the
    /// draft or the document moves, and says it is checking until the
    /// staged run lands.
    fn split_plane_rows(&mut self, ui: &mut egui::Ui, tool: &crate::combine::SplitTool) {
        if tool.seats().read(0).is_none() {
            return;
        }
        if let Some((doc, evaluation)) = self.session.landed_pair()
            && let Some(read) = tool.seats().read(0)
            && crate::splitplane::source(doc, evaluation, &read)
                .is_some_and(|source| crate::splitplane::centre(source).is_none())
        {
            crate::widgets::message_toned(ui, SPLIT_NO_CENTRE, &self.theme, Tone::Advisory);
        }
        if tool.seats().through(0).is_none() {
            crate::widgets::message_toned(ui, SPLIT_NOT_AIMED, &self.theme, Tone::Advisory);
        } else if self.split_plane_preview().is_none() {
            crate::widgets::message_toned(ui, SPLIT_NOT_DRAWN, &self.theme, Tone::Advisory);
        }
        let op = (|| {
            tool.op(
                self.notation
                    .length_literals(self.drafts.split_origin)
                    .ok()?,
                scalars(self.drafts.split_normal).ok()?,
            )
            .ok()
        })();
        let Some(Ok(creation)) = op.map(Creation::of) else {
            return;
        };
        let staged = self.session.staged_for(&creation);
        let said = match staged {
            None | Some(Staged::Checking) => None,
            Some(Staged::Refused(refusal)) => Some(refusal.to_string()),
            Some(Staged::Evaluated {
                node, evaluation, ..
            }) => match tree::split_reading(evaluation, node) {
                Ok(None) => None,
                Ok(Some(empty)) => Some(format!("{SPLIT_LEAVES} `{empty}`: {SPLIT_MISSES}")),
                Err(standing) => Some(standing.to_string()),
            },
        };
        match (staged.is_some(), said) {
            (false, _) => {
                self.ops.push(SessionOp::Stage { creation });
                crate::widgets::message(ui, SPLIT_CHECKING);
                ui.ctx().request_repaint();
            }
            (true, Some(said)) => {
                crate::widgets::message_toned(ui, said, &self.theme, Tone::Advisory);
            }
            (true, None) if matches!(staged, Some(Staged::Checking)) => {
                crate::widgets::message(ui, SPLIT_CHECKING);
                // A staged answer is not a user event, so nothing else
                // would wake the frame loop to collect it.
                ui.ctx().request_repaint();
            }
            (true, None) => {
                crate::widgets::message(ui, SPLIT_CUTS);
            }
        }
    }

    /// The mirror tool's panel: the body and the face it is reflected
    /// across, held as picks ([`crate::combine::MirrorTool`]), and the
    /// one committed edit.
    pub(crate) fn mirror_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.mirror().cloned() else {
            return self.tool_door(ui, ToolKind::Mirror);
        };
        crate::widgets::message(
            ui,
            ToolKind::Mirror.says(
                &"pick a face: its body mirrors across it — pick another face to mirror across that one; a drawn body becomes its symmetric part, the body and its image fused",
            ),
        );
        crate::widgets::message_toned(
            ui,
            tool.line(
                self.session.doc(),
                frame::landed_speaker(self.session.landed_pair()),
            ),
            &self.theme,
            Tone::Advisory,
        );
        let waiting = tool
            .seats()
            .awaits_half(0)
            .then(|| (0, Seat::MirrorBody.name().to_owned()));
        if let Some((seat, half)) = half_rows(ui, waiting, &self.theme) {
            self.tools.choose_half(seat, half);
        }
        self.tool_commit_row(ui, ToolKind::Mirror, |_, _| Ok(tool.op()?));
    }

    /// The transform tool's panel: one body pick plus the placement
    /// fields, and the one committed edit.
    pub(crate) fn transform_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.transform() else {
            return self.tool_door(ui, ToolKind::Transform);
        };
        crate::widgets::message(ui, ToolKind::Transform.says(&"pick the body to place"));
        if let Some((seat, half)) = seats_row(ui, tool.seats(), self.session.doc(), &self.theme) {
            self.tools.choose_half(seat, half);
        }
        ui.horizontal(|ui| {
            unit_vec3_row(
                ui,
                "translation",
                self.notation.length.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.transform_translation,
            );
            length_picker(ui, "transform_translation", &mut self.notation.length);
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
                self.notation.angle.def(),
                ANGLE_DRAG_SPEED,
                &mut self.drafts.transform_angle,
            );
            angle_picker(ui, "transform_angle", &mut self.notation.angle);
        });
        self.tool_commit_row(ui, ToolKind::Transform, |drafts, notation| {
            Ok(tool.op(
                notation.length_literals(drafts.transform_translation)?,
                scalars(drafts.transform_axis)?,
                notation.angle_literal(drafts.transform_angle)?,
            )?)
        });
    }

    /// The pattern tool's panel: a body pick, a rule choice with its
    /// fields, the axis pick the circular rule needs, the output
    /// choice, and the one committed edit.
    ///
    /// The output row is what fuses a pattern into the part: `fused`
    /// commits the family with a `Union` over it, ONE body, where
    /// `instances` commits the family of several bodies.
    pub(crate) fn pattern_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.pattern() else {
            return self.tool_door(ui, ToolKind::Pattern);
        };
        crate::widgets::message(
            ui,
            ToolKind::Pattern
                .says(&"pick the body; a circular rule's axis is in its own coordinates"),
        );
        if let Some((seat, half)) = seats_row(ui, tool.seats(), self.session.doc(), &self.theme) {
            self.tools.choose_half(seat, half);
        }
        ui.horizontal(|ui| {
            ui.label("rule");
            for (kind, label) in PatternLayoutChoice::ALL {
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
            PatternLayoutChoice::Linear => {
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
                        self.notation.length.def(),
                        FIELD_DRAG_SPEED,
                        &mut self.drafts.pattern_spacing,
                    );
                    length_picker(ui, "pattern_spacing", &mut self.notation.length);
                });
            }
            PatternLayoutChoice::Circular => {
                ui.horizontal(|ui| {
                    unit_vec3_row(
                        ui,
                        "axis through",
                        self.notation.length.def(),
                        FIELD_DRAG_SPEED,
                        &mut self.drafts.pattern_through,
                    );
                    length_picker(ui, "pattern_through", &mut self.notation.length);
                });
                vec3_row(
                    ui,
                    "axis direction",
                    UNIT_DRAG_SPEED,
                    &mut self.drafts.pattern_axis,
                );
                ui.horizontal(|ui| {
                    ui.label("step");
                    unit_field(
                        ui,
                        self.notation.angle.def(),
                        ANGLE_DRAG_SPEED,
                        &mut self.drafts.pattern_step,
                    );
                    angle_picker(ui, "pattern_step", &mut self.notation.angle);
                });
            }
        }
        self.tool_commit_row(ui, ToolKind::Pattern, |drafts, notation| {
            match drafts.pattern_kind {
                PatternLayoutChoice::Linear => Ok(tool.linear_op(
                    drafts.pattern_output,
                    drafts.pattern_count,
                    scalars(drafts.pattern_direction)?,
                    notation.length_literal(drafts.pattern_spacing)?,
                )?),
                PatternLayoutChoice::Circular => Ok(tool.circular_op(
                    drafts.pattern_output,
                    drafts.pattern_count,
                    notation.length_literals(drafts.pattern_through)?,
                    scalars(drafts.pattern_axis)?,
                    notation.angle_literal(drafts.pattern_step)?,
                )?),
            }
        });
    }

    /// The projection tool's panel — [`crate::combine::PartTool`],
    /// which authors a `Node::Part`: one pick of a pattern, the index
    /// field, and the one committed edit.
    ///
    /// Called the PROJECTION tool on screen because "part" is already
    /// the word the `Add part…` chooser beside it uses for another
    /// document, and the two gestures have nothing to do with each
    /// other.
    pub(crate) fn projection_tool_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tool) = self.tools.part() else {
            return self.tool_door(ui, ToolKind::Part);
        };
        crate::widgets::message(
            ui,
            ToolKind::Part.says(&"pick a pattern in the tree to project one of its copies"),
        );
        if let Some((seat, half)) = seats_row(ui, tool.seats(), self.session.doc(), &self.theme) {
            self.tools.choose_half(seat, half);
        }
        part_selector_rows(ui, &self.theme, &mut self.drafts.part_instance);
        self.tool_commit_row(ui, ToolKind::Part, |drafts, _| {
            Ok(tool.instance_op(drafts.part_instance)?)
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
            return self.tool_door(ui, ToolKind::Duplicate);
        };
        crate::widgets::message(ui, ToolKind::Duplicate.says(&"pick the body to duplicate"));
        if let Some((seat, half)) = seats_row(ui, tool.seats(), self.session.doc(), &self.theme) {
            self.tools.choose_half(seat, half);
        }
        crate::widgets::message_toned(ui, duplicate_note(), &self.theme, Tone::Advisory);
        self.tool_commit_row(ui, ToolKind::Duplicate, |_, _| Ok(tool.op()?));
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
            return self.tool_door(ui, ToolKind::Blend);
        };
        crate::widgets::message(ui, ToolKind::Blend.says(&"pick the edges to blend"));
        crate::widgets::message_toned(ui, FREEZE_NOTE, &self.theme, Tone::Advisory);
        ui.weak(match (target, self.session.landed_pair()) {
            (Some(target), Some((landed, _))) => format!(
                "{count} edges picked on {}",
                Said(&target, Speaker::of(landed))
            ),
            (Some(target), None) => format!("{count} edges picked on {target}"),
            (None, _) => "no edges picked yet".to_owned(),
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
                self.notation.length.def(),
                FIELD_DRAG_SPEED,
                &mut self.drafts.blend_size,
            );
            length_picker(ui, "blend_size", &mut self.notation.length);
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
        let ready = target.zip(self.session.landed_pair()).zip(self.index);
        let Some(((target, (landed, eval)), index)) = ready else {
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
            .and_then(|tool| tool.load_all_edges(target, landed, eval, index));
        if let Some(event) = event {
            self.notices.push(frame::tool_notice(
                &ToolNotice::Blend(event),
                Some((landed, eval)),
            ));
        }
    }

    /// The blend tool's commit/cancel row — [`ViewerBehavior::tool_commit_row`]'s
    /// rules, spelled here because this tool holds a set rather than
    /// seats. With no edge held there is no body to blend and so no op:
    /// the commit is disabled and says why.
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
        let noun = self.tool_noun(ToolKind::Blend);
        if let Some(noun) = noun {
            self.creation_label_row(ui, noun);
        }
        let mut clear = false;
        let closing = commit_row(
            ui,
            ToolKind::Blend.commit(),
            (count == 0).then_some("pick an edge first"),
            |ui| clear = clear_picks_button(ui, count, Picked::Edges),
        );
        if clear && let Some(tool) = self.tools.blend_mut() {
            tool.clear();
        }
        if closing.commit {
            match self.notation.length_literal(self.drafts.blend_size) {
                Ok(size) => {
                    let op: Option<SessionOp> =
                        self.tools
                            .blend()
                            .and_then(|tool| match self.drafts.blend_kind {
                                BlendKindChoice::Fillet => tool.fillet_op(size.clone()),
                                BlendKindChoice::Chamfer => tool.chamfer_op(size),
                            });
                    if let Some(op) = op {
                        self.push_tool_op(noun, op);
                    }
                }
                Err(error) => {
                    self.notices
                        .push(frame::tool_news(&ToolKind::Blend.refuses(&error)));
                }
            }
        }
        if closing.cancel {
            self.tools.close();
        }
    }

    /// **A closed tool's door** ([`door`]): one modal tool at a time,
    /// so the click goes to `Tools::open`, which closes whatever was
    /// open — the rule and its argument live in that value rather than
    /// at each door.
    fn tool_door(&mut self, ui: &mut egui::Ui, kind: ToolKind) {
        if door(ui, kind.button()).clicked() {
            self.tools.open(kind);
        }
    }

    /// **The kind noun of the node a tool's commit creates** —
    /// `node_kind_noun`'s word, which its proposed label counts by —
    /// or `None` for the mate tool, whose commit is a constraint in a
    /// placement's bundle and creates no node, so its form offers no
    /// label ([`Self::tool_commit_row`]). It is read off the tool, not
    /// the op ([`SessionOp::creates_a_node`]), because the label is
    /// offered before the picks make an op; a noun beside an op that
    /// creates no node is said at the commit ([`Self::push_labelled`]).
    /// The
    /// pattern and blend tools create one of two kinds, by the choice
    /// their form holds, and the boolean tool one of three, by the
    /// operation it holds; the duplicate tool's last node is a
    /// projection, and the mirror tool's, for a drawn body, the union
    /// of its symmetric part; the label lands there.
    fn tool_noun(&self, kind: ToolKind) -> Option<&'static str> {
        Some(match kind {
            ToolKind::Mate => return None,
            ToolKind::Revolve => "Revolve",
            ToolKind::Boolean => match self.tools.boolean().map(|tool| tool.operation()) {
                Some(BooleanOp::Union) | None => "Union",
                Some(BooleanOp::Intersect) => "Intersect",
                Some(BooleanOp::Subtract) => "Subtract",
            },
            ToolKind::Split => "Split",
            // A drawn body becomes its symmetric part, whose last node
            // is the union; an unplaced body's image stands alone.
            ToolKind::Mirror => match self.tools.mirror().and_then(|tool| tool.seats().read(0)) {
                Some(body)
                    if !crate::world::placements_of_read(self.session.committed_doc(), &body)
                        .is_empty() =>
                {
                    "Union"
                }
                _ => "Mirror",
            },
            ToolKind::Transform => "Transform",
            ToolKind::Pattern => match self.drafts.pattern_output {
                PatternOutputChoice::Instances => "Place",
                PatternOutputChoice::Fused => "Union",
            },
            ToolKind::Blend => match self.drafts.blend_kind {
                BlendKindChoice::Fillet => "Fillet",
                BlendKindChoice::Chamfer => "Chamfer",
            },
            ToolKind::Part | ToolKind::Duplicate => "Part",
        })
    }

    /// **A create form's label field** (DESIGN.md Band 1, "Node
    /// labels"): the text its node will be labelled with, editable.
    /// Until the person types, it shows the proposal for this moment
    /// ([`tree::proposed_label`], `Kind N`); nothing is stored until
    /// the form commits ([`Self::push_labelled`]), and a field left
    /// blank commits no label.
    fn creation_label_row(&mut self, ui: &mut egui::Ui, noun: &'static str) {
        let mut text = self.creation_label_text(noun);
        ui.horizontal(|ui| {
            ui.label("label");
            if ui.text_edit_singleline(&mut text).changed() {
                self.drafts.creation_labels.insert(noun, text);
            }
        });
    }

    /// What a create form's label field holds: what was typed, or the
    /// proposal for this moment.
    fn creation_label_text(&self, noun: &'static str) -> String {
        self.drafts
            .creation_labels
            .get(noun)
            .cloned()
            .unwrap_or_else(|| {
                tree::proposed_label(self.session.doc(), noun)
                    .map(|label| label.as_str().to_owned())
                    .unwrap_or_default()
            })
    }

    /// **A create form's commit, with its label**: `op` queued as
    /// [`SessionOp::CreateLabelled`] when the form's label field holds
    /// a label — the insert and the label as one undo — or alone when
    /// it is blank. The field's draft is kept until the creation
    /// commits (`Drafts::creation_landed`), so a refused creation does
    /// not cost what was typed. A text the label rule refuses queues
    /// nothing and is said on the status line.
    fn push_labelled(&mut self, noun: &'static str, op: SessionOp) {
        let text = self.creation_label_text(noun);
        match crate::drafts::label_typed(&text) {
            Ok(label) => {
                match (label, Creation::of(op)) {
                    (Some(label), Ok(creation)) => {
                        self.ops.push(SessionOp::CreateLabelled { creation, label });
                    }
                    (None, Ok(creation)) => self.ops.push(creation.into_op()),
                    // Every form that reaches here creates a node; one
                    // that does not is committed as it is and said.
                    (_, Err(op)) => {
                        self.notices.push(frame::tool_news(
                            &FormKind::Label.refuses(&"this form creates no node to label"),
                        ));
                        self.ops.push(*op);
                    }
                }
            }
            Err(fault) => self
                .notices
                .push(frame::tool_news(&FormKind::Label.refuses(&fault))),
        }
    }

    /// A tool's commit: labelled when the tool creates a node
    /// ([`Self::tool_noun`]), queued alone when it creates none.
    fn push_tool_op(&mut self, noun: Option<&'static str>, op: SessionOp) {
        match noun {
            Some(noun) => self.push_labelled(noun, op),
            None => self.ops.push(op),
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
        kind: ToolKind,
        op: impl FnOnce(&Drafts, Notation) -> Result<SessionOp, CommitFault>,
    ) {
        let noun = self.tool_noun(kind);
        if let Some(noun) = noun {
            self.creation_label_row(ui, noun);
        }
        let closing = commit_row(ui, kind.commit(), None, |_| {});
        if closing.commit {
            match op(self.drafts, *self.notation) {
                Ok(op) => self.push_tool_op(noun, op),
                Err(error) => {
                    self.notices.push(frame::tool_news(&kind.refuses(&error)));
                }
            }
        }
        if closing.cancel {
            self.tools.close();
        }
    }
}

/// **What a tool's picks are, for its `Clear picks`**: the words its
/// two states say on the hover egui shows in each.
#[derive(Clone, Copy)]
enum Picked {
    /// The blend tool's edges.
    Edges,
    /// The boolean tool's bodies.
    Bodies,
}

impl Picked {
    /// What a click does, live.
    fn drops(self) -> &'static str {
        match self {
            Self::Edges => "drop every picked edge and start on any body",
            Self::Bodies => "drop every picked body",
        }
    }

    /// Why the button is disabled with nothing held.
    fn nothing(self) -> &'static str {
        match self {
            Self::Edges => "no edge is picked, so there is nothing to clear",
            Self::Bodies => "no body is picked, so there is nothing to clear",
        }
    }
}

/// **A tool's `Clear picks`**, live while `count` picks are held. No
/// operation stands behind it — clearing is tool state — so with
/// nothing held there is no refusal to read, and [`Picked`] carries the
/// sentence: each state its own, on the hook egui shows in that state.
///
/// Answers whether it was clicked, which a disabled button never is.
fn clear_picks_button(ui: &mut egui::Ui, count: usize, picked: Picked) -> bool {
    let button = ui.add_enabled(count > 0, egui::Button::new("Clear picks"));
    if count > 0 {
        button.on_hover_text(picked.drops()).clicked()
    } else {
        button.on_disabled_hover_text(picked.nothing()).clicked()
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

    use pncad::document::{Doc, ProfileProgram, RecipeNodeId, Speaker};
    use pncad::geom_core::Tol;
    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};

    use super::{
        Picked, clear_picks_button, duplicate_note, mate_picks_row, part_selector_rows, seats_row,
    };
    use crate::combine::BooleanTool;
    use crate::frame::Tone;
    use crate::matetool::MateTool;
    use crate::pane::headless::{painted_after_clicking, painted_text, painted_while_hovering};
    use crate::session::FaceSelection;
    use crate::theme::Theme;
    use pncad::select::SplitHalf;
    use pncad::topo::BooleanOp;

    /// The end cap `minted` made, picked on the body of `drawn`.
    fn cap_on(minted: u64, drawn: u64) -> FaceSelection {
        FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(minted)),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(drawn)),
            body: 0,
        }
    }

    /// **The mate panel's picks, painted**: the seated tools' line —
    /// one `role: pick` item per side, `—` for the open one, the
    /// seated line's empty sentence — with each pick said as the chrome
    /// says a pick: the face in its leaf role's words, and the body it
    /// was picked on where that is not the feature the words name.
    #[test]
    fn the_mate_panel_says_its_picks_in_the_seated_panels_line() {
        // The picks' nodes are not in this document, so each is spoken
        // as a bare tag.
        let doc = Doc::<ProfileProgram>::empty_derived("mate-row", Tol::witness());
        let painted = |tool: &MateTool| {
            painted_text(|ui| mate_picks_row(ui, tool, Speaker::of(&doc), &Theme::DEFAULT))
        };
        let mut tool = MateTool::new();
        assert_eq!(painted(&tool), "no picks yet");
        tool.pick(&doc, cap_on(3, 3));
        assert_eq!(
            painted(&tool),
            "on: the end cap of node 000000000003; to: —"
        );
        tool.pick(&doc, cap_on(3, 5));
        assert_eq!(
            painted(&tool),
            "on: the end cap of node 000000000003; \
             to: the end cap of node 000000000003, on node 000000000005"
        );
    }

    /// **The boolean panel's picks, painted**: a union's members in
    /// pick order, any number of them; a subtraction's two seats by
    /// role, the body KEPT named before the second is picked.
    #[test]
    fn the_boolean_panel_lists_members_and_names_subtracts_seats() {
        let doc = Doc::<ProfileProgram>::empty_derived("seats-row", Tol::witness());
        let node = |n| RecipeNodeId::new(0, test_utils::refusal::tagged(n));
        let mut tool = BooleanTool::new();
        let painted = |tool: &BooleanTool| {
            painted_text(|ui| {
                crate::widgets::message_toned(ui, tool.line(&doc), &Theme::DEFAULT, Tone::Advisory);
            })
        };
        assert_eq!(painted(&tool), "no picks yet");
        for n in [3, 5, 7] {
            tool.pick(&doc, node(n));
        }
        assert_eq!(
            painted(&tool),
            "member 1: node 000000000003; member 2: node 000000000005; \
             member 3: node 000000000007"
        );
        let mut subtract = BooleanTool::new();
        subtract.set_operation(BooleanOp::Subtract);
        subtract.pick(&doc, node(3));
        assert_eq!(
            painted(&subtract),
            "body kept: node 000000000003; body removed: —"
        );
        subtract.pick(&doc, node(5));
        assert_eq!(
            painted(&subtract),
            "body kept: node 000000000003; body removed: node 000000000005"
        );
    }

    /// **A body seat holding a split picked in the tree offers its two
    /// halves**, and a click answers that seat and the half it chose —
    /// what the panel hands the open tool.
    #[test]
    fn a_seat_holding_a_split_offers_its_halves() {
        use crate::test_support::inserted;
        let tol = Tol::witness();
        let (doc, block, _) = crate::test_support::boss_on_block("seats-half", tol);
        let tool = editor_core::test_support::plane_on(
            &doc,
            block,
            [0.0, 0.0, 0.001],
            [0.0, 0.0, 1.0],
            Tol::witness(),
        );
        let (doc, split) = inserted(
            &doc,
            pncad::document::Node::Split {
                target: block.into(),
                tool,
            },
            tol,
        );
        let mut tool = crate::combine::TransformTool::new();
        tool.pick(&doc, split);
        let mut chosen = None;
        let drawn = painted_after_clicking(crate::tree::split_half_label(SplitHalf::Below), |ui| {
            if let Some(choice) = seats_row(ui, tool.seats(), &doc, &Theme::DEFAULT) {
                chosen = Some(choice);
            }
        });
        assert!(drawn.contains("half not chosen"), "{drawn}");
        assert_eq!(chosen, Some((0, SplitHalf::Below)), "{drawn}");
    }

    /// The part form paints the index field with its numbering
    /// sentence.
    #[test]
    fn the_part_form_paints_the_index() {
        let mut instance = 3_i64;
        let drawn = painted_text(|ui| part_selector_rows(ui, &Theme::DEFAULT, &mut instance));
        assert!(drawn.contains("instance"), "{drawn}");
        assert!(
            drawn.contains("numbered from zero"),
            "the field says where the numbering starts: {drawn}"
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
        assert!(
            note.contains("along its own (1, 0, 0)"),
            "along the original's own +x: {note}"
        );
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
    /// picture** — the copy it selects is placed and drawn, and a person
    /// should hear that before the click, not discover it after.
    #[test]
    fn the_part_form_says_the_projection_is_placed() {
        let mut instance = 1_i64;
        let drawn = painted_text(|ui| part_selector_rows(ui, &Theme::DEFAULT, &mut instance));
        assert!(
            drawn.contains("the selected copy is placed in the world and drawn"),
            "{drawn}"
        );
        assert!(
            drawn.contains("pick it again in the feature tree"),
            "and where the rest can still be reached from: {drawn}"
        );
    }

    /// **`Clear picks` says why while it is disabled** — hovered with
    /// nothing held, it paints its own sentence, and not the enabled
    /// one that egui would have shown nobody.
    #[test]
    fn clear_picks_with_nothing_held_says_there_is_nothing_to_clear() {
        let hovered = painted_while_hovering("Clear picks", 0, |ui| {
            clear_picks_button(ui, 0, Picked::Edges);
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
            clear_picks_button(ui, 3, Picked::Edges);
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
    use pncad::document::DocumentId;

    use super::{part_entry, part_window};
    use crate::pane::headless::{
        SLACK, assert_inside, assert_own_lines, assert_under, drawn_in, find, landed_after,
        painted_while_hovering,
    };
    use crate::parts::PartEntry;
    use crate::theme::Theme;

    /// A narrow pane, and wider than `crate::widgets::message_floor`,
    /// so these rows read the region and not the floor.
    const REGION: f32 = 260.0;

    #[test]
    fn a_parts_id_is_said_under_its_file_name_inside_the_pane() {
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

    /// **A part's file name is bounded by the pane, and whole on
    /// hover**: one line inside the pane however long the name, and the
    /// full name in the tooltip of a pointer resting on it.
    #[test]
    fn a_long_file_name_is_elided_inside_the_pane_and_whole_on_hover() {
        let entry = PartEntry {
            id: DocumentId(7),
            path: PathBuf::from(
                "outer-enclosure-lid-with-the-long-hinge-and-the-second-gasket-groove.pncad",
            ),
            open_document: false,
        };
        let (region, painted) = drawn_in(REGION, |ui| {
            part_entry(ui, &Theme::DEFAULT, &entry);
        });
        let name = find(&painted, &entry.file_name());
        assert_eq!(name.rows.len(), 1, "one line, not {:?}", name.rows);
        assert_inside(region, name);

        let hovered = painted_while_hovering(&entry.file_name(), 0, |ui| {
            ui.allocate_ui(egui::vec2(REGION, 800.0), |ui| {
                part_entry(ui, &Theme::DEFAULT, &entry);
            });
        });
        assert_eq!(
            hovered.matches(&entry.file_name()).count(),
            2,
            "the elided name and the whole one in its tooltip: {hovered}"
        );
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

    use pncad::document::DocumentId;

    use super::{Withheld, bore_withholds, part_listing, withheld_for, withheld_line};
    use crate::pane::headless::{Landed, Voices, find, find_opening, landed_voiced};
    use crate::parts::{PartCensus, PartChooser, PartEntry};
    use crate::session::Refusal;
    use crate::theme::Theme;

    fn listed(
        dir: Option<&str>,
        offered: Result<Vec<PartEntry>, Refusal>,
    ) -> (Vec<Landed>, Voices) {
        let chooser = PartChooser::opened(PartCensus::taken(dir.map(PathBuf::from), offered));
        landed_voiced(&Theme::DEFAULT, |ui, theme| {
            part_listing(ui, theme, &chooser);
        })
    }

    /// **A chooser with no directory says so once, loud** — the
    /// refusal's own sentence, and no quiet "no directory" over it.
    #[test]
    fn a_chooser_with_no_directory_says_so_once_and_loud() {
        let (painted, voices) = listed(None, Err(Refusal::NoDocumentDirectory));
        assert_eq!(
            find_opening(&painted, "save the document first").ink,
            Some(voices.actionable)
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
            Some(voices.actionable)
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

    /// **A bore at least as wide as the radius is a refused input**, and
    /// loud — not the form asking for its next one.
    #[test]
    fn a_bore_as_wide_as_the_radius_is_refused() {
        assert!(matches!(
            bore_withholds(true, 0.01, 0.01),
            Some(Withheld::Refused(_))
        ));
        assert_eq!(bore_withholds(true, 0.005, 0.01), None);
        assert_eq!(bore_withholds(false, 0.02, 0.01), None);
    }

    /// **Which withholding reason is said**: a refused input over the form
    /// waiting for one, wherever it came in the form; between two of
    /// one kind, the earlier.
    #[test]
    fn a_refused_input_outranks_a_missing_one_and_the_form_order_breaks_ties() {
        let frame = Withheld::Waiting("pick a face to draw on");
        let chain = Withheld::Waiting("add a step to the chain");
        let bore = Withheld::Refused("the bore is too wide");
        let wider = Withheld::Refused("the bore is wider still");
        assert_eq!(withheld_for([frame, chain]), Some(frame));
        assert_eq!(withheld_for([frame, bore]), Some(bore));
        assert_eq!(withheld_for([bore, frame]), Some(bore));
        assert_eq!(withheld_for([frame, bore, chain]), Some(bore));
        assert_eq!(withheld_for([bore, wider]), Some(bore));
        assert_eq!(withheld_for([]), None);
    }

    /// **The add-profile form's withholding reason**: a refused input is loud,
    /// the form waiting for one is quiet.
    #[test]
    fn a_withheld_profile_button_says_why_in_the_reasons_own_voice() {
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            withheld_line(ui, theme, Withheld::Refused("the bore is too wide"));
            withheld_line(ui, theme, Withheld::Waiting("choose a shape to add"));
        });
        assert_eq!(
            find(&painted, "the bore is too wide").ink,
            Some(voices.actionable)
        );
        assert_eq!(
            find(&painted, "choose a shape to add").ink,
            Some(voices.weak)
        );
    }
}

/// **A form's noun is the kind of the node its op mints**: the
/// proposed label counts by the noun, so the noun has to be
/// `node_kind_noun`'s word for what the form commits.
#[cfg(test)]
mod creation_nouns {
    #![allow(clippy::expect_used)]

    use pncad::document::{
        ContentPin, Doc, DocRef, DocumentId, Node, ProfileProgram, node_kind_noun,
    };
    use pncad::geom_core::Tol;

    use super::{EXTRUDE_NOUN, INSTANCE_NOUN, PROFILE_NOUN};
    use crate::session::{DocSession, SessionOp};
    use crate::test_support::{len, square_profile};

    /// The node `op` made on `session` ([`crate::world::made`]), by its
    /// kind noun.
    fn minted_noun(session: &mut DocSession, op: SessionOp) -> &'static str {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        let node = crate::world::made(session.doc(), &outcome.minted).expect("the op made a node");
        node_kind_noun(session.doc().node(node).expect("the minted node is live"))
    }

    #[test]
    fn the_profile_and_extrude_forms_nouns_are_their_nodes_kinds() {
        let tol = Tol::witness();
        let doc: Doc<ProfileProgram> = Doc::empty_derived("creation-nouns", tol);
        let (doc, profile) = square_profile(&doc, 0.02, tol);
        let mut session = DocSession::inline(doc, tol);
        let extrude = SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        };
        assert_eq!(minted_noun(&mut session, extrude), EXTRUDE_NOUN);
        let add_profile = SessionOp::AddProfile {
            loops: vec![crate::test_support::rectangle_loop([0.0, 0.0], 0.01, 0.01)],
        };
        assert_eq!(minted_noun(&mut session, add_profile), PROFILE_NOUN);
    }

    /// The part chooser's noun is the kind of the node `AddInstance`
    /// inserts, so its proposal counts the document's instances.
    #[test]
    fn the_part_choosers_noun_is_the_instances_kind() {
        let instance = Node::<ProfileProgram>::instantiate_part(
            DocRef {
                id: DocumentId(7),
                pin: ContentPin([0; 32]),
            },
            pncad::document::VarId::WORLD,
            Vec::new(),
        );
        assert_eq!(node_kind_noun(&instance), INSTANCE_NOUN);
    }
}

/// **The add-profile form's held face across a selection that moved
/// on** ([`ViewerBehavior::sketch_face_rows`]'s pick, and
/// `Drafts::respeak` after each op).
#[cfg(test)]
mod sketch_face_said {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use pncad::document::{Label, Speaker};
    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};

    use crate::drafts::Drafts;
    use crate::session::{DocSession, FaceSelection, Selection, SessionOp};

    /// **The add-profile form keeps its held face's nodes past the
    /// selection**: once the selection moves on, the form still says
    /// its face's minting node by the last label it had, after a rename
    /// and a delete, where the selection's snapshot no longer names it.
    #[test]
    fn a_held_sketch_face_says_its_deleted_node_by_its_last_label() {
        let tol = pncad::tolerance::witness();
        let (doc, block, boss) = crate::test_support::boss_on_block("sketch-face-said", tol);
        let t = test_utils::refusal::tag(block.0.digest());
        let face = FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: block,
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node: block,
            body: 0,
        };
        let mut session = DocSession::inline(doc, tol);
        let mut drafts = Drafts::default();
        let perform = |session: &mut DocSession, drafts: &mut Drafts, op| {
            let outcome = session.perform(op);
            assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
            drafts.respeak(session.doc());
        };
        perform(
            &mut session,
            &mut drafts,
            SessionOp::SetLabel {
                node: block,
                label: Some(Label::new("base").expect("a label")),
            },
        );
        session.pump();
        perform(
            &mut session,
            &mut drafts,
            SessionOp::Select(Selection::Face(face.clone())),
        );
        drafts.hold_sketch_face(face, session.selection_said());
        perform(
            &mut session,
            &mut drafts,
            SessionOp::Select(Selection::Node(boss)),
        );
        for op in [
            SessionOp::SetLabel {
                node: block,
                label: Some(Label::new("plinth").expect("a label")),
            },
            SessionOp::DeleteNode { node: block },
        ] {
            perform(&mut session, &mut drafts, op);
        }
        session.pump();
        let (landed, _) = session.landed_pair().expect("the delete landed");
        assert!(
            landed.node(block).is_none(),
            "the landed run lost the block"
        );
        let said = |kept| Speaker::of(landed).or_held(kept).node(block).to_string();
        assert_eq!(
            said(drafts.sketch_face_said()),
            format!("Extrude \"plinth\" ({t})"),
            "the form says the held face's node by its last label"
        );
        assert_eq!(
            said(session.selection_said()),
            format!("node {t}"),
            "the selection moved on, so its snapshot does not name the block"
        );
    }
}

/// **An open tool's held nodes across a rename and a delete**
/// (`Tools::respeak` after each op): the tool names a node it lost by
/// the last label the document gave it, not the one it had at the pick.
#[cfg(test)]
mod tools_respeak {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used, clippy::panic)]

    use pncad::document::{Label, MateSide, RecipeNodeId};
    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};

    use crate::session::{DocSession, EdgeSelection, FaceSelection, Selection, SessionOp};
    use crate::tools::{ToolKind, ToolNotice, Tools};

    /// A tool of `kind` that picked `pick(block, boss)` while the block
    /// was labelled `base`, then the block renamed `plinth` and deleted,
    /// with the tools re-spoken after every op as the op loop does.
    fn picked_renamed_deleted(
        kind: ToolKind,
        pick: impl Fn(RecipeNodeId, RecipeNodeId) -> Vec<Selection>,
    ) -> (DocSession, Tools, String) {
        let tol = pncad::tolerance::witness();
        let (doc, block, boss) = crate::test_support::boss_on_block("tools-respeak", tol);
        let t = test_utils::refusal::tag(block.0.digest());
        let mut session = DocSession::inline(doc, tol);
        let mut tools = Tools::new();
        let perform = |session: &mut DocSession, tools: &mut Tools, op| {
            let outcome = session.perform(op);
            assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
            tools.respeak(session.doc());
        };
        let label = |text| SessionOp::SetLabel {
            node: block,
            label: Some(Label::new(text).expect("a label")),
        };
        perform(&mut session, &mut tools, label("base"));
        session.pump();
        tools.open(kind);
        let picks: Vec<SessionOp> = pick(block, boss)
            .into_iter()
            .map(SessionOp::Select)
            .collect();
        let index = {
            let (doc, eval) = session.landed_pair().expect("the block landed");
            let generation = session.landed_generation().expect("a landed generation");
            crate::pickindex::PickIndex::build(
                doc,
                eval,
                crate::pickindex::PictureKey::of(generation, crate::test_support::plate_delta()),
                session.tol(),
            )
            .expect("the block indexes")
        };
        let declined = tools.feed(session.doc(), Ok(&index), &picks);
        assert!(declined.is_empty(), "every pick is taken: {declined:?}");
        for op in [label("plinth"), SessionOp::DeleteNode { node: block }] {
            perform(&mut session, &mut tools, op);
        }
        session.pump();
        let (landed, _) = session.landed_pair().expect("the delete landed");
        assert!(
            landed.node(block).is_none(),
            "the landed run lost the block"
        );
        (session, tools, format!("Extrude \"plinth\" ({t})"))
    }

    /// The tool's drops, as the survival step reports them.
    fn dropped(session: &DocSession, tools: &mut Tools) -> Vec<ToolNotice> {
        tools.reconcile(session.doc(), session.landed_pair())
    }

    fn face(node: RecipeNodeId) -> FaceSelection {
        FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node,
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node,
            body: 0,
        }
    }

    /// Every seated tool, each a fresh fixture: one arm each in
    /// `Tools::respeak`.
    #[test]
    fn a_seated_tool_says_its_lost_pick_by_its_last_label() {
        for kind in [
            ToolKind::Revolve,
            ToolKind::Boolean,
            ToolKind::Split,
            ToolKind::Transform,
            ToolKind::Pattern,
            ToolKind::Part,
            ToolKind::Duplicate,
        ] {
            let (session, mut tools, last) =
                picked_renamed_deleted(kind, |block, _| vec![Selection::Node(block)]);
            let said: Vec<String> = dropped(&session, &mut tools)
                .into_iter()
                .map(|notice| match notice {
                    ToolNotice::Seated { event, .. } => {
                        let crate::seats::SeatEvent::PickLost { node, .. } = event;
                        node.to_string()
                    }
                    other => panic!("a seated tool's drop, not {other:?}"),
                })
                .collect();
            assert_eq!(
                said,
                [last],
                "the {kind:?} seat names the block by its last label"
            );
        }
    }

    /// The mirror tool's one face pick holds the body and the plane,
    /// and both drops name the block by its last label.
    #[test]
    fn the_mirror_tool_says_its_lost_body_and_plane_by_their_last_label() {
        let (session, mut tools, last) = picked_renamed_deleted(ToolKind::Mirror, |block, _| {
            vec![Selection::Face(face(block))]
        });
        let said: Vec<String> = dropped(&session, &mut tools)
            .into_iter()
            .map(|notice| match notice {
                ToolNotice::Seated { event, .. } => {
                    let crate::seats::SeatEvent::PickLost { node, .. } = event;
                    node.to_string()
                }
                ToolNotice::Mirror(crate::combine::MirrorEvent::PlaneLost {
                    node, said, ..
                }) => said.spoken(node).to_string(),
                other => panic!("a mirror drop, not {other:?}"),
            })
            .collect();
        assert_eq!(
            said,
            [last.clone(), last],
            "the body seat, then the plane, each by the block's last label"
        );
    }

    #[test]
    fn the_blend_tool_says_its_lost_target_by_its_last_label() {
        let (session, mut tools, last) = picked_renamed_deleted(ToolKind::Blend, |block, _| {
            vec![Selection::Edge(EdgeSelection {
                name: StableName {
                    kind: EntityKind::Edge,
                    node: block,
                    path: vec![RoleSeg::Cap(CapEnd::End)],
                },
                node: block,
                body: 0,
            })]
        });
        let said: Vec<String> = dropped(&session, &mut tools)
            .into_iter()
            .map(|notice| match notice {
                ToolNotice::Blend(crate::blend::BlendEvent::TargetLost { node, .. }) => {
                    node.to_string()
                }
                other => panic!("the target's drop, not {other:?}"),
            })
            .collect();
        assert_eq!(said, [last], "the target is named by its last label");
    }

    /// The instance-pick refusal is `app`'s
    /// `the_mate_refusal_says_the_kept_label_while_a_delete_has_not_landed`,
    /// where the survival step has not dropped the pick.
    #[test]
    fn the_mate_tool_says_its_lost_pick_by_its_last_label() {
        let (session, mut tools, last) = picked_renamed_deleted(ToolKind::Mate, |block, boss| {
            vec![Selection::Face(face(block)), Selection::Face(face(boss))]
        });
        let said: Vec<String> = dropped(&session, &mut tools)
            .into_iter()
            .filter_map(|notice| match notice {
                ToolNotice::Mate(crate::matetool::MateToolEvent::PickLost {
                    side: MateSide::On,
                    pick,
                    said,
                    ..
                }) => Some(said.spoken(pick.node).to_string()),
                ToolNotice::Mate(_) => None,
                other => panic!("a mate drop, not {other:?}"),
            })
            .collect();
        assert_eq!(
            said,
            [last],
            "the on pick's drop names the block by its last label"
        );
    }
}
