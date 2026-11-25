//! The View pane: display tolerance, the camera's state, and the
//! show/hide toggle of every space and body
//! ([`crate::display::DisplayState::toggles`]).
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::quantity::{LengthUnit, UnitDef};

use pncad::document::{Doc, ProfileProgram, RecipeNodeId, VarId};

use crate::app::ViewerBehavior;
use crate::camera::Camera;
use crate::display::SpaceToggles;
use crate::frame;
use crate::props::Notation;
use crate::scene::DisplayTolerance;
use crate::session::SessionOp;

impl ViewerBehavior<'_> {
    /// The view pane: the numbers the camera and the tessellation are
    /// actually running at.
    pub(crate) fn view_ui(&mut self, ui: &mut egui::Ui) {
        let stats = self.scene.stats();
        ui.heading("View");
        // One δ, because there is only ever one: the budget chose it
        // when the document opened or the user did, and the note says
        // which. The triangle count below is the picture's own, so
        // nothing here is a prediction.
        self.delta_ui(ui);
        if self.budget_delta.is_some() {
            crate::widgets::message_toned(
                ui,
                "chosen for the triangle budget; δ is yours from here",
                &self.theme,
                frame::Tone::Advisory,
            );
        }
        ui.label(format!("faces: {}", stats.faces));
        ui.label(format!("triangles: {}", stats.triangles));
        ui.separator();
        // The COMMITTED document, the one the toggles were read over.
        let doc = self.session.committed_doc();
        let part_files = self.session.part_files();
        let label = |node| {
            crate::relations::Related::node(doc, node, part_files)
                .said()
                .clone()
        };
        self.ops.extend(toggles_ui(
            ui,
            doc,
            self.session.doc(),
            &self.toggles.groups,
            label,
        ));
        ui.separator();
        for line in camera_readout(self.camera, *self.notation) {
            ui.label(line);
        }
        ui.separator();
        ui.label(format!("history: {} states", self.session.history().len()));
        match self.session.path() {
            // A path, which nothing bounds, so `widgets::message`.
            Some(path) => crate::widgets::message(ui, format!("file: {}", path.display())),
            None => ui.weak("unsaved document"),
        };
        if let Some(status) = self.status.as_ref() {
            ui.separator();
            // A sentence, so `widgets::sentence` — here in a layout
            // egui would have wrapped it in anyway, which is a fact
            // about the column and not about the message.
            let said = crate::widgets::sentence(ui, status.said(), self.session.doc(), None);
            self.ops.extend(said.map(crate::session::SessionOp::Select));
        }
    }

    /// The δ control: the display tolerance as a number the user types,
    /// in the working notation's length unit.
    pub(crate) fn delta_ui(&mut self, ui: &mut egui::Ui) {
        delta_field(
            ui,
            self.delta,
            self.notation.length,
            &mut self.drafts.delta_text,
            self.notices,
            self.delta_request,
        );
    }
}

/// What the View pane says beside a body the default picture leaves
/// out: an operation reads it as a body, and draws it there.
pub(crate) const READ_AS_A_BODY: &str = "read by an operation";

/// What a body's toggle says on hover while its space is hidden.
pub(crate) const SPACE_HIDDEN: &str = "its space is hidden";

/// The View pane's label for the bodies in no space.
pub(crate) const IN_NO_SPACE: &str = "in no space";

/// **The toggles, drawn**: each space's checkbox, and under it each
/// body's, named by `label` — and the display ops the clicks name.
fn toggles_ui(
    ui: &mut egui::Ui,
    doc: &Doc<ProfileProgram>,
    on_screen: &Doc<ProfileProgram>,
    groups: &[SpaceToggles],
    label: impl Fn(RecipeNodeId) -> pncad::document::Sentence,
) -> Vec<SessionOp> {
    let mut ops = Vec::new();
    for group in groups {
        match group.space {
            Some(space) => {
                let mut on = group.on;
                if ui.checkbox(&mut on, space_label(doc, space)).changed() {
                    ops.push(SessionOp::SetSpaceHidden { space, hidden: !on });
                }
            }
            None => {
                ui.label(IN_NO_SPACE);
            }
        }
        for body in &group.bodies {
            ui.horizontal(|ui| {
                ui.add_space(crate::pane::features::INDENT_STEP);
                let mut on = body.on;
                // A body under a hidden space keeps its own toggle,
                // which the space's then overrides. The box toggles and
                // the name beside it is a link to the body, so the two
                // clicks are two widgets.
                let toggle = ui
                    .add_enabled(group.on, egui::Checkbox::without_text(&mut on))
                    .on_disabled_hover_text(SPACE_HIDDEN);
                let name = crate::widgets::sentence(ui, &label(body.node), on_screen, None);
                ops.extend(name.map(SessionOp::Select));
                if !body.drawn_by_default {
                    ui.weak(READ_AS_A_BODY);
                }
                if toggle.changed() {
                    ops.push(SessionOp::SetBodyOn {
                        node: body.node,
                        on,
                    });
                }
            });
        }
    }
    ops
}

/// A space as its toggle names it: as a sentence does, capitalised.
fn space_label(doc: &Doc<ProfileProgram>, space: VarId) -> String {
    crate::tools::capitalised(
        &pncad::document::Said(&doc.spoken_space(space), pncad::document::Speaker::TAG)
            .sentence()
            .to_string(),
    )
}

/// The camera's state as the View pane's two lines, read in the working
/// `notation` like every other value nobody wrote: the angles in its
/// angle unit, the distances in its length unit, each through
/// [`camera_reading`].
fn camera_readout(camera: &Camera, notation: Notation) -> [String; 2] {
    let angle = notation.angle.def();
    let length = notation.length.def();
    [
        format!(
            "camera yaw {}, pitch {}",
            camera_reading(camera.yaw(), angle),
            camera_reading(camera.pitch(), angle)
        ),
        format!(
            "distance {} (band {}–{})",
            camera_reading(camera.distance(), length),
            camera_reading(camera.min_distance(), length),
            camera_reading(camera.max_distance(), length)
        ),
    ]
}

/// One reading of the camera — a distance or an angle — in `unit`, as
/// text a person reads.
///
/// **It ASKS for the value in `unit` rather than forming it**
/// ([`crate::props::written_text`]). A camera distance above
/// `f64::MAX * MILLI` metres has no millimetre value at all and reads
/// in metres; `inf` names no distance, and
/// [`crate::props::no_reading`] is the answer to one.
///
/// **That is the half of this render owns, and there is another it
/// does not.** `Camera::max_distance` is
/// `scene_radius * MAX_DISTANCE_FACTOR`, and `Camera::new` admits
/// every finite `scene_radius`, so from about `1.798e306` m up the
/// band's top arrives here ALREADY infinite — a value this function
/// can only report, since no bound inside a render reaches a product
/// formed above it. The bound belongs at `Camera::new`, beside the
/// finiteness check it already runs on that argument, and is filed as
/// `camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite`.
fn camera_reading(canonical: f64, unit: UnitDef) -> String {
    crate::props::written_text(canonical, unit)
}

/// The δ field's inner margin, set rather than inherited so
/// [`delta_text_edit`]'s width is a sum of two numbers this file names.
const FIELD_MARGIN: egui::Margin = egui::Margin::symmetric(4, 2);

/// **The δ field, one width whatever it holds.**
///
/// The text area is [`crate::widgets::widest_number`] in the field's
/// own font, which a told number fits
/// (`the_field_shows_every_render_the_bound_covers` measures it), plus
/// [`FIELD_MARGIN`]. δ's exact spelling
/// ([`crate::scene::DisplayTolerance::field_text_in`]) is as long as
/// its figures and its zeros make it, up to hundreds of characters, and
/// a field as wide as that would push the pane out; so a longer text
/// scrolls inside the field (egui's single-line edit follows its
/// cursor) rather than widening it
/// (`the_field_keeps_its_width_over_a_long_spelling`).
///
/// A pane narrower than this clips anyway; that is every field in the
/// chrome and is not this width's to fix.
fn delta_text_edit<'text>(ui: &egui::Ui, text: &'text mut String) -> egui::TextEdit<'text> {
    let font = egui::FontSelection::Default.resolve(ui.style());
    let room = crate::widgets::widest_number(ui, &font);
    egui::TextEdit::singleline(text)
        .margin(FIELD_MARGIN)
        .desired_width(room + FIELD_MARGIN.sum().x)
}

/// The δ field: the display tolerance as a number the user types, in
/// `unit` (the working notation's), writing a committed δ into
/// `delta_request` and a refusal into `notices`.
///
/// **A text field rather than a pair of step buttons.** δ is a
/// LENGTH, and the question a user has is "how fine" — a
/// halve/double pair answers it only by repeated clicking and
/// cannot reach a number in between. It is also not a `DragValue`:
/// a drag would commit a tessellation per frame, which is the one
/// mistake [`crate::widgets::drag_ops`] exists to keep out of this file.
///
/// Committing on lost focus covers Enter too — egui's singleline
/// field surrenders focus on Enter — so there is one commit path,
/// not two. What is typed is a DRAFT until then: nothing
/// re-tessellates while a number is half-entered, and `0.` on the
/// way to `0.05` never reaches the tessellator.
///
/// **A keystroke is what makes a draft, and nothing else does.** The
/// text a field shows while nobody has typed into it is a RENDER of
/// the δ in force, its exact spelling in `unit`
/// ([`crate::scene::DisplayTolerance::field_text_in`]); the text a field
/// commits is a draft, which is the user's own spelling. Holding the
/// two apart is what `draft: Option<String>` is for
/// ([`crate::drafts::Drafts::delta_text`]): `Some` is text as typed, so
/// a field that was focused and left with nothing typed into it has
/// nothing to commit and commits nothing — no re-tessellation, and no
/// trip through the float the typed number door reads.
///
/// **A draft typed back to the render is that same untouched field,
/// two keystrokes later.** The render is what the box already held, so
/// a draft that reads as it carries no number the render does not, and
/// it commits nothing. What a user who means to re-assert the displayed
/// δ does instead is type any other spelling of it — `0.050` for a
/// render of `0.05` — which is a draft like any other and commits.
fn delta_field(
    ui: &mut egui::Ui,
    in_force: DisplayTolerance,
    unit: LengthUnit,
    draft: &mut Option<String>,
    notices: &mut Vec<frame::Message>,
    delta_request: &mut Option<DisplayTolerance>,
) {
    let drafted = draft.is_some();
    let render = in_force.field_text_in(unit);
    let mut text = draft.take().unwrap_or_else(|| render.clone());
    let field = ui
        .horizontal(|ui| {
            let field = ui.add(delta_text_edit(ui, &mut text));
            ui.label(format!("{} display δ", unit.symbol()));
            field
        })
        .inner;
    // egui reports `changed` for a text mutation and for nothing else,
    // so this is the keystroke the paragraph above turns on. A field
    // already holding a draft keeps it on the frames between two
    // keystrokes.
    if drafted || field.changed() {
        *draft = Some(text);
    }
    if field.lost_focus()
        && let Some(typed) = draft.take()
    {
        let typed = typed.trim();
        // Whitespace around the render is still the render: it reads as
        // the same number, so it is the same no-op.
        if typed != render {
            match typed.parse::<f64>() {
                // Judged by `DisplayTolerance`, not here: a δ that is
                // not a finite positive length is refused at that one
                // door, wherever it came from, and its refusal names
                // the number typed in the field's unit.
                Ok(written) => match DisplayTolerance::typed(written, unit) {
                    Ok(delta) => *delta_request = Some(delta),
                    Err(error) => notices.push(frame::delta_refusal(&error)),
                },
                Err(error) => {
                    notices.push(frame::delta_not_a_number(typed, &error));
                }
            }
        }
    }
    // The draft lives at most as long as the focus does, so a δ that
    // moved under the field (the budget's choice on open) shows up in
    // it.
    if !field.has_focus() {
        *draft = None;
    }
}

/// **What the δ field does with the focus, driven through a real
/// `egui::Context`.**
///
/// The rows that matter here are about the difference between a field
/// that was TYPED into and one that was merely visited, and nothing
/// short of egui's own focus lifecycle tells those apart: `changed`,
/// `lost_focus` and `has_focus` are all egui's answers, so a stub for
/// them would be a test of the stub. Each row runs frames against a
/// headless context, the δ field and one other focusable widget to
/// take the focus away again.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{READ_AS_A_BODY, camera_reading, camera_readout, delta_field, toggles_ui};
    use crate::camera::Camera;
    use crate::display::{BodyToggle, SpaceToggles};
    use crate::frame;
    use crate::pane::headless::painted_text;
    use crate::props::Notation;
    use crate::scene::DisplayTolerance;
    use crate::session::SessionOp;
    use eframe::egui;
    use pncad::document::{Doc, RecipeNodeId, VarId};
    use pncad::geom_core::Tol;
    use pncad::quantity::{DEG, IN, LengthUnit, M, MM, PI};

    /// One body, off, that the default leaves out, under the world.
    fn one_undrawn_body() -> (RecipeNodeId, Vec<SpaceToggles>) {
        let node = RecipeNodeId::new(0, test_utils::refusal::tagged(7));
        let groups = vec![SpaceToggles {
            space: Some(VarId::WORLD),
            on: true,
            bodies: vec![BodyToggle {
                node,
                on: false,
                drawn_by_default: false,
            }],
        }];
        (node, groups)
    }

    /// **The list names the world first, a body under it, and says
    /// beside a body the default leaves out that an operation reads
    /// it**; a click on that body's toggle asks to show it.
    ///
    /// *Breaks if* the world's row is not drawn above its bodies, the
    /// note is dropped, or the click pushes a hide (or nothing).
    #[test]
    fn the_toggle_list_names_its_rows_and_a_click_shows_an_undrawn_body() {
        let doc = Doc::empty_derived("view-toggles", Tol::witness());
        let (node, groups) = one_undrawn_body();
        let text = painted_text(|ui| {
            toggles_ui(ui, &doc, &doc, &groups, |_| "the body".into());
        });
        let world = text.find("The world").expect("the world's row is drawn");
        let body = text.find("the body").expect("the body's row is drawn");
        assert!(world < body, "the world first: {text}");
        assert!(text.contains(READ_AS_A_BODY), "{text}");

        let named = crate::pane::headless::find(
            &crate::pane::headless::landed(|ui| {
                toggles_ui(ui, &doc, &doc, &groups, |_| "the body".into());
            }),
            "the body",
        )
        .allocated;
        // The box, left of the name it toggles.
        let at = egui::pos2(
            named.left() - egui::Style::default().spacing.interact_size.y / 2.0,
            named.center().y,
        );
        assert_eq!(
            crate::pane::headless::ops_after_clicking_at(at, |ui| {
                toggles_ui(ui, &doc, &doc, &groups, |_| "the body".into())
            }),
            vec![SessionOp::SetBodyOn { node, on: true }],
            "the click shows the body"
        );
    }

    /// **A body's name in the list is a link to it**: a click on the
    /// name selects the body and toggles nothing.
    ///
    /// *Breaks if* the name is drawn as the checkbox's own text (a click
    /// on it toggles) or as words.
    #[test]
    fn a_bodys_name_in_the_list_selects_it() {
        let (doc, _) = crate::scene::plate_with_hole(pncad::tolerance::witness())
            .expect("the startup document");
        let extrude = doc.ids()[1];
        let groups = vec![SpaceToggles {
            space: Some(VarId::WORLD),
            on: true,
            bodies: vec![BodyToggle {
                node: extrude,
                on: true,
                drawn_by_default: true,
            }],
        }];
        let files = crate::parts::PartFiles::default();
        let label = |node| {
            crate::relations::Related::node(&doc, node, &files)
                .said()
                .clone()
        };
        let name = crate::tree::node_label(&doc, extrude, &files);
        let at = crate::pane::headless::find(
            &crate::pane::headless::landed(|ui| {
                toggles_ui(ui, &doc, &doc, &groups, label);
            }),
            &name,
        )
        .allocated
        .center();
        assert_eq!(
            crate::pane::headless::ops_after_clicking_at(at, |ui| {
                toggles_ui(ui, &doc, &doc, &groups, label)
            }),
            vec![SessionOp::Select(crate::session::Selection::Node(extrude))],
            "the click selects the body"
        );
    }

    /// **The camera readout reads in the unit it is handed and asks
    /// whether the value exists there.**
    ///
    /// The unit: the same distance reads in metres and millimetres, and
    /// an angle in degrees and half turns, as the unit table writes
    /// them. A camera distance above `f64::MAX * MILLI` metres has no
    /// millimetre float, and the render says it in millimetres all the
    /// same, never spelling `inf`.
    #[test]
    fn a_camera_reading_reads_in_the_unit_it_is_handed() {
        assert_eq!(camera_reading(0.05, M.def()), "0.05 m");
        assert_eq!(camera_reading(0.05, MM.def()), "50 mm");
        assert_eq!(
            camera_reading(core::f64::consts::FRAC_PI_2, DEG.def()),
            "90 deg"
        );
        assert_eq!(
            camera_reading(core::f64::consts::FRAC_PI_2, PI.def()),
            "0.5 pi rad"
        );
        let unnameable = 1.0e306;
        assert_eq!(camera_reading(unnameable, MM.def()), "1e309 mm");
        assert_eq!(camera_reading(unnameable, M.def()), "1e306 m");
        assert!(
            !camera_reading(f64::INFINITY, M.def()).contains("inf"),
            "and a band top that arrives already infinite is still not spelled as a distance"
        );
    }

    /// **The View pane's camera lines follow the working notation** —
    /// the text `view_ui` labels, read off one camera under two
    /// notations, so a line that kept a fixed unit, or read its angles
    /// in the length unit, reds on one of them.
    #[test]
    fn the_camera_readout_follows_the_working_notation() {
        use core::f64::consts::{FRAC_PI_2, FRAC_PI_4};

        let camera = Camera::new(
            pncad::geom_core::Point3::new(0.0, 0.0, 0.0),
            2.0,
            FRAC_PI_2,
            FRAC_PI_4,
            1.0,
            1.0,
        )
        .expect("a camera over a unit scene");
        assert_eq!(
            camera_readout(&camera, Notation::DEFAULT),
            [
                "camera yaw 0.5 pi rad, pitch 0.25 pi rad",
                "distance 2 m (band 0.05 m–100 m)",
            ],
        );
        let millimetres_and_degrees = Notation {
            length: MM,
            angle: DEG,
        };
        assert_eq!(
            camera_readout(&camera, millimetres_and_degrees),
            [
                "camera yaw 90 deg, pitch 45 deg",
                "distance 2000 mm (band 50 mm–100000 mm)",
            ],
        );
    }

    /// One δ field, one button to tab the focus onto, and the three
    /// values the field writes.
    struct Field {
        ctx: egui::Context,
        delta: DisplayTolerance,
        unit: LengthUnit,
        draft: Option<String>,
        notices: Vec<frame::Message>,
        request: Option<DisplayTolerance>,
    }

    impl Field {
        fn at(delta_mm: f64) -> Self {
            Self {
                ctx: egui::Context::default(),
                delta: DisplayTolerance::new(delta_mm * 1.0e-3).expect("a positive δ"),
                unit: MM,
                draft: None,
                notices: Vec::new(),
                request: None,
            }
        }

        /// One frame, with `events` delivered to it.
        fn frame(&mut self, events: Vec<egui::Event>) {
            let ctx = self.ctx.clone();
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            let (delta, unit) = (self.delta, self.unit);
            let draft = &mut self.draft;
            let notices = &mut self.notices;
            let request = &mut self.request;
            let mut output = ctx.run_ui(input, |ui| {
                delta_field(ui, delta, unit, draft, notices, request);
                let _ = ui.button("elsewhere");
            });
            // The font atlas is built on the first pass and epaint
            // panics on a dropped delta nobody uploaded; no test here
            // paints.
            output.textures_delta.clear();
        }

        fn tab(&mut self) {
            self.frame(vec![egui::Event::Key {
                key: egui::Key::Tab,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }]);
        }

        fn type_text(&mut self, text: &str) {
            self.frame(vec![egui::Event::Text(text.to_owned())]);
        }

        fn backspace(&mut self) {
            self.frame(vec![egui::Event::Key {
                key: egui::Key::Backspace,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }]);
        }
    }

    /// The focus has to actually land, or every row below passes
    /// vacuously.
    #[test]
    fn tabbing_onto_the_field_and_typing_reaches_the_draft() {
        let mut field = Field::at(0.05);
        field.frame(Vec::new());
        field.tab();
        field.type_text("7");
        assert!(
            field.draft.is_some(),
            "a keystroke in the focused field makes a draft"
        );
    }

    /// A δ below half a micrometre — `0.0004` mm, which the render
    /// carries as a decimal and `{:.3}` carried as `0.000`. Visiting the
    /// field must not commit anything at all, whatever it reads as.
    #[test]
    fn focus_and_leave_below_half_a_micrometre_commits_nothing() {
        let mut field = Field::at(0.000_4);
        field.frame(Vec::new());
        field.tab();
        field.tab();
        assert_eq!(
            field.request, None,
            "a field nobody typed into has nothing to commit"
        );
        assert!(field.notices.is_empty(), "and nothing to refuse");
        assert!(field.draft.is_none());
    }

    /// The same, above the sub-micrometre band, where the old failure
    /// was silent: δ = 1.6 µm, which `{:.3}` carried as `0.002`.
    #[test]
    fn focus_and_leave_does_not_quantise_the_delta_in_force() {
        let mut field = Field::at(0.001_6);
        field.frame(Vec::new());
        field.tab();
        field.tab();
        assert_eq!(field.request, None, "δ is unchanged by a visit");
        assert!(field.notices.is_empty());
    }

    /// A δ the user DID type still commits, and through the field's
    /// one commit path — Enter, which egui answers by surrendering the
    /// focus.
    #[test]
    fn a_typed_delta_commits_on_enter() {
        let mut field = Field::at(0.05);
        field.frame(Vec::new());
        field.tab();
        field.frame(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            egui::Event::Text("0.02".to_owned()),
        ]);
        assert_eq!(field.request, None, "a draft is not a commit");
        field.frame(vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        assert_eq!(
            field.request,
            Some(DisplayTolerance::new(0.02 * 1.0e-3).expect("a positive δ")),
            "and Enter commits it"
        );
        assert!(field.draft.is_none(), "the draft is spent");
    }

    /// `0.` on the way to `0.05` never reaches the tessellator: a
    /// half-typed number is a draft and writes no request.
    #[test]
    fn a_half_typed_number_is_not_committed() {
        let mut field = Field::at(0.05);
        field.frame(Vec::new());
        field.tab();
        field.type_text("0");
        field.type_text(".");
        assert_eq!(field.request, None, "nothing re-tessellates mid-entry");
        // The two keystrokes, appended to the render the field was
        // showing when the focus arrived — `0.05`, this δ as it is told
        // in millimetres.
        assert_eq!(field.draft.as_deref(), Some("0.050."));
    }

    /// **A draft typed back to the render commits nothing.** The render
    /// is what the field already held, so two keystrokes that cancel out
    /// leave the user's own draft reading exactly as the render did, and
    /// δ is left as it stands. δ here is 1/30 of the starting δ, whose
    /// render is every one of its figures.
    #[test]
    fn a_draft_typed_back_to_the_render_commits_nothing() {
        let mut field = Field::at(0.05 / 30.0);
        field.frame(Vec::new());
        field.tab();
        field.type_text("7");
        assert!(field.draft.is_some(), "the keystroke made a draft");
        field.backspace();
        assert_eq!(
            field.draft.as_deref(),
            Some("0.0016666666666666669"),
            "and the draft is back to the render it started from"
        );
        field.tab();
        assert_eq!(
            field.request, None,
            "a draft that reads as the render is not a δ the user asked for"
        );
        assert!(field.notices.is_empty(), "and nothing to refuse");
    }

    /// The other side of that rule: any OTHER spelling of the displayed
    /// δ is a draft like any other and commits, so re-asserting the
    /// number on screen has a route.
    #[test]
    fn another_spelling_of_the_rendered_delta_still_commits() {
        let mut field = Field::at(0.05);
        field.frame(Vec::new());
        field.tab();
        field.frame(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            // `0.05` is the render; `0.0500` is not, and reads the same.
            egui::Event::Text("0.0500".to_owned()),
        ]);
        field.tab();
        assert_eq!(
            field.request,
            Some(DisplayTolerance::new(0.05 * 1.0e-3).expect("a positive δ"))
        );
    }

    /// **The field can show every render the character bound covers.** A
    /// render wider than the box is clipped, and a clipped render reads
    /// as a different δ — so the field [`super::delta_text_edit`] builds is
    /// measured against egui's own font metrics for every text
    /// `crate::readout::widest_render` bounds: its character count of
    /// each of `crate::readout::GLYPHS`, which covers the two widest
    /// spellings `number` returns whichever glyph is the font's widest.
    ///
    /// The chrome sets no text styles of its own, so the headless
    /// context's metrics are the application's.
    #[test]
    fn the_field_shows_every_render_the_bound_covers() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            for character in crate::readout::GLYPHS.chars() {
                let mut text: String =
                    std::iter::repeat_n(character, crate::readout::MAX_CHARS + 1).collect();
                let shown = super::delta_text_edit(ui, &mut text).show(ui);
                let room = shown.response.rect.width() - super::FIELD_MARGIN.sum().x;
                let wanted = shown.galley.size().x;
                assert!(
                    wanted <= room,
                    "{} '{character}' need {wanted} points and the field offers {room}",
                    crate::readout::MAX_CHARS + 1
                );
            }
        });
        output.textures_delta.clear();
    }

    /// **A spelling longer than the bound does not widen the field**:
    /// the field over a told number's widest text and the field over a
    /// ninety-figure spelling are one width.
    #[test]
    fn the_field_keeps_its_width_over_a_long_spelling() {
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            let mut bound: String =
                std::iter::repeat_n('0', crate::readout::MAX_CHARS + 1).collect();
            let mut long = format!("0.{}1", "0".repeat(88));
            let bounded = super::delta_text_edit(ui, &mut bound)
                .show(ui)
                .response
                .rect;
            let longer = super::delta_text_edit(ui, &mut long).show(ui).response.rect;
            assert_eq!(
                longer.width(),
                bounded.width(),
                "a long spelling widened the field"
            );
        });
        output.textures_delta.clear();
    }

    /// **The field reads and writes the working notation's unit**: the
    /// same δ shows as its metre value in a metre notation, and a
    /// number typed there commits as metres — so a field that kept
    /// millimetres reds on both halves.
    #[test]
    fn the_field_reads_and_commits_in_the_working_unit() {
        let mut field = Field::at(0.05);
        field.unit = M;
        field.frame(Vec::new());
        field.tab();
        field.type_text("7");
        assert_eq!(
            field.draft.as_deref(),
            Some("0.000057"),
            "the render the keystroke landed on is δ in metres, positionally"
        );
        field.frame(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            egui::Event::Text("0.002".to_owned()),
        ]);
        field.tab();
        assert_eq!(
            field.request,
            Some(DisplayTolerance::new(0.002).expect("a positive δ")),
            "and a typed number is metres"
        );
    }

    /// **The field's seed is δ's exact spelling in the field's unit,
    /// never in exponent form**, below `1e-4` and past `1e16` of the
    /// unit as between them, and it types back as δ: a keystroke
    /// lands after the seed's last digit, so the draft it makes reads
    /// as the seed with that digit appended.
    #[test]
    fn the_fields_seed_is_delta_positionally_and_types_back() {
        for (delta_m, unit, seed) in [
            (5.0e-5, M, "0.00005"),
            (1.0e-9, M, "0.000000001"),
            (4.0e-8, MM, "0.00004"),
            (2.5e14, MM, "250000000000000000"),
            (3.0e17, M, "300000000000000000"),
            (0.0254e-6, IN, "0.000001"),
        ] {
            let delta = DisplayTolerance::new(delta_m).expect("a positive δ");
            assert_eq!(
                delta.field_text_in(unit),
                seed,
                "δ = {delta_m} m in {unit:?}"
            );
            let mut field = Field::at(0.05);
            (field.delta, field.unit) = (delta, unit);
            field.frame(Vec::new());
            field.tab();
            field.type_text("7");
            assert_eq!(
                field.draft.as_deref(),
                Some(format!("{seed}7").as_str()),
                "the edit opened on {seed}"
            );
            let typed: f64 = seed.parse().expect("the seed is a number");
            assert_eq!(
                DisplayTolerance::typed(typed, unit)
                    .ok()
                    .map(DisplayTolerance::get),
                Some(delta.get()),
                "{seed} {unit:?} types back as δ"
            );
        }
    }

    /// **A refused δ is named as it was typed, in the field's unit** —
    /// the number on the screen and the symbol beside it, not the
    /// world-unit value the door judged. Both of the door's arms, each
    /// under a notation that is not the default, so a refusal that
    /// echoed metres reds on the number and one that dropped the unit
    /// reds on the symbol.
    #[test]
    fn a_refused_delta_is_named_in_the_unit_it_was_typed_in() {
        for (unit, typed, said) in [
            (MM, "-2", "-2 mm is not a finite, strictly positive"),
            (IN, "1e308", "1e308 in is past the coarsest"),
        ] {
            let mut field = Field::at(0.05);
            field.unit = unit;
            field.frame(Vec::new());
            field.tab();
            field.frame(vec![
                egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND,
                },
                egui::Event::Text(typed.to_owned()),
            ]);
            field.tab();
            assert_eq!(field.request, None, "{typed} {unit:?} commits nothing");
            let said_texts: Vec<String> = field.notices.iter().map(frame::Message::text).collect();
            assert!(
                matches!(said_texts.as_slice(), [one] if one.starts_with(said)),
                "{typed} in {unit:?} is refused as typed: {said_texts:?}"
            );
        }
    }

    /// A δ that moved under an unfocused field shows up in it, because
    /// an unfocused field holds no text of its own.
    #[test]
    fn a_delta_that_moved_under_the_field_is_shown() {
        let mut field = Field::at(0.05);
        field.frame(Vec::new());
        field.tab();
        field.tab();
        field.delta = DisplayTolerance::new(0.012 * 1.0e-3).expect("a positive δ");
        field.frame(Vec::new());
        assert!(
            field.draft.is_none(),
            "nothing stands between the field and the δ in force"
        );
    }
}
