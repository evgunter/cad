//! The View pane: display tolerance, datums and the camera's state.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::quantity::{LengthUnit, UnitDef};

use crate::app::ViewerBehavior;
use crate::camera::Camera;
use crate::frame;
use crate::props::Notation;
use crate::scene::DisplayTolerance;

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
        // **Datum visibility, and why it is a switch at all.**
        // Construction geometry is drawn over the part, which is
        // where it has to be for a plane to say what it cuts — and it
        // is also in the way once a document has several. A view
        // setting rather than a document one: which datums exist is
        // the recipe's business, and whether this window draws them is
        // this window's.
        ui.checkbox(self.show_datums, "show datums");
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
            // A sentence, so `widgets::message` — here in a layout
            // egui would have wrapped it in anyway, which is a fact
            // about the column and not about the message.
            crate::widgets::message(ui, status.text());
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
/// `f64::MAX * MILLI` metres has no millimetre value at all, and `inf`
/// names no distance; [`crate::props::written`] is the question and
/// [`crate::props::no_reading`] is the answer.
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

/// **The δ field, as wide as the widest render it can be handed.**
///
/// A render the field cannot show is clipped, and a clipped render
/// reads as a different δ — the defect [`crate::readout::MAX_CHARS`]
/// exists to prevent — so the text area is
/// [`crate::widgets::widest_number`] in the field's own font, and the
/// field is that plus [`FIELD_MARGIN`].
/// `the_field_shows_every_render_the_bound_covers` measures it.
///
/// A pane narrower than this clips anyway; that is every field in the
/// chrome and is not this width's to fix.
fn delta_text_edit<'text>(ui: &egui::Ui, text: &'text mut String) -> egui::TextEdit<'text> {
    let font = egui::FontSelection::Default.resolve(ui.style());
    egui::TextEdit::singleline(text)
        .margin(FIELD_MARGIN)
        .desired_width(crate::widgets::widest_number(ui, &font) + FIELD_MARGIN.sum().x)
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
/// the δ in force, shortened to read as a length; the text a field
/// commits is a draft, which is the user's own spelling. Holding the
/// two apart is what `draft: Option<String>` is for
/// ([`crate::drafts::Drafts::delta_text`]): `Some` is text as typed, so
/// a field that was focused and left with nothing typed into it has
/// nothing to commit and commits nothing. Seeding the draft with the
/// render instead would make an untouched field indistinguishable from
/// a typed one, and every δ whose render is not its own exact spelling
/// would move — silently, or into
/// [`crate::scene::DisplayTolerance::new`]'s refusal — because a field
/// was focused and left.
///
/// **A draft typed back to the render is that same untouched field,
/// two keystrokes later.** The render is what the box already held, so
/// a draft that reads as it carries no number the render does not; and
/// since the render is a rounding of δ
/// ([`crate::scene::DisplayTolerance::render_in`]), committing one
/// could only move δ to a coarser spelling of itself. There is no δ
/// for which that is what the user asked for, so it commits nothing.
/// What a user who means to re-assert the displayed δ does instead is
/// type any other spelling of it — `0.050` for a render of `0.05` —
/// which is a draft like any other and commits.
fn delta_field(
    ui: &mut egui::Ui,
    in_force: DisplayTolerance,
    unit: LengthUnit,
    draft: &mut Option<String>,
    notices: &mut Vec<frame::Message>,
    delta_request: &mut Option<DisplayTolerance>,
) {
    let drafted = draft.is_some();
    let render = in_force.render_in(unit);
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

    use super::{camera_reading, camera_readout, delta_field};
    use crate::camera::Camera;
    use crate::frame;
    use crate::props::Notation;
    use crate::scene::DisplayTolerance;
    use eframe::egui;
    use pncad::quantity::{DEG, IN, LengthUnit, M, MM, PI};

    /// **The camera readout reads in the unit it is handed and asks
    /// whether the value exists there.**
    ///
    /// The unit: the same distance reads in metres and millimetres, and
    /// an angle in degrees and half turns, as the unit table writes
    /// them. The existence: a camera distance above `f64::MAX * MILLI`
    /// metres has no millimetre value, and the render says which
    /// notation could not name it instead of spelling `inf`, while
    /// metres still name it.
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
        assert_eq!(camera_reading(unnameable, MM.def()), "no mm reading");
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
        // showing when the focus arrived — `0.05`, the shortest decimal
        // spelling that reads back as this δ.
        assert_eq!(field.draft.as_deref(), Some("0.050."));
    }

    /// **A draft typed back to the render commits nothing.** The render
    /// is what the field already held, so two keystrokes that cancel out
    /// leave the user's own draft reading exactly as the render did —
    /// and committing a render can only coarsen δ to a spelling of
    /// itself. δ here is 1/30 of the starting δ, whose render
    /// (`0.0016666667`) is a rounding: committing it would move δ from
    /// 1.66666666666…e-6 to 1.6666667e-6.
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
            Some("0.0016666667"),
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
            "the render the keystroke landed on is δ in metres"
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
            let said_texts: Vec<&str> = field.notices.iter().map(frame::Message::text).collect();
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
