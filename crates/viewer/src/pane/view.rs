//! The View pane: display tolerance, datums and the camera's state.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;

use crate::app::ViewerBehavior;
use crate::frame;
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
            ui.weak("chosen for the triangle budget; δ is yours from here");
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
            ui.label(status.text());
        }
    }

    /// The δ control: the display tolerance as a number the user types,
    /// in millimetres.
    pub(crate) fn delta_ui(&mut self, ui: &mut egui::Ui) {
        delta_field(
            ui,
            self.delta,
            &mut self.drafts.delta_mm,
            self.notices,
            self.delta_request,
        );
    }
}

/// How wide the δ field is, in points.
///
/// Wide enough for the longest text
/// [`crate::scene::DisplayTolerance::render_mm`] can return, because a
/// render the field cannot show is clipped — and a clipped render reads
/// as a different δ, which is the defect the render's own bound exists
/// to prevent. `the_field_shows_the_longest_render` measures it against
/// egui's own font metrics rather than asserting it in prose. A pane
/// narrower than this clips anyway; that is every field in the chrome
/// and is not this number's to fix.
const FIELD_WIDTH: f32 = 88.0;

/// The δ field: the display tolerance as a number the user types, in
/// millimetres, writing a committed δ into `delta_request` and a
/// refusal into `notices`.
///
/// **A text field rather than a pair of step buttons.** δ is a
/// LENGTH, and the question a user has is "how fine, in mm" — a
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
/// ([`crate::drafts::Drafts::delta_mm`]): `Some` is text as typed, so
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
/// ([`crate::scene::DisplayTolerance::render_mm`]), committing one
/// could only move δ to a coarser spelling of itself. There is no δ
/// for which that is what the user asked for, so it commits nothing.
/// What a user who means to re-assert the displayed δ does instead is
/// type any other spelling of it — `0.050` for a render of `0.05` —
/// which is a draft like any other and commits.
fn delta_field(
    ui: &mut egui::Ui,
    in_force: DisplayTolerance,
    draft: &mut Option<String>,
    notices: &mut Vec<frame::Message>,
    delta_request: &mut Option<f64>,
) {
    let drafted = draft.is_some();
    let render = in_force.render_mm();
    let mut text = draft.take().unwrap_or_else(|| render.clone());
    let field = ui
        .horizontal(|ui| {
            let field = ui.add(egui::TextEdit::singleline(&mut text).desired_width(FIELD_WIDTH));
            ui.label("mm display δ");
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
                // door, wherever it came from.
                Ok(mm) => *delta_request = Some(mm * 1.0e-3),
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

    use super::delta_field;
    use crate::frame;
    use crate::scene::DisplayTolerance;
    use eframe::egui;

    /// One δ field, one button to tab the focus onto, and the three
    /// values the field writes.
    struct Field {
        ctx: egui::Context,
        delta: DisplayTolerance,
        draft: Option<String>,
        notices: Vec<frame::Message>,
        request: Option<f64>,
    }

    impl Field {
        fn at(delta_mm: f64) -> Self {
            Self {
                ctx: egui::Context::default(),
                delta: DisplayTolerance::new(delta_mm * 1.0e-3).expect("a positive δ"),
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
            let delta = self.delta;
            let draft = &mut self.draft;
            let notices = &mut self.notices;
            let request = &mut self.request;
            let mut output = ctx.run_ui(input, |ui| {
                delta_field(ui, delta, draft, notices, request);
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
        assert_eq!(field.request, Some(0.02 * 1.0e-3), "and Enter commits it");
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
    /// (`0.001667`) is a rounding: committing it would move δ from
    /// 1.6666…e-6 to 1.667e-6.
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
            Some("0.001667"),
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
        assert_eq!(field.request, Some(0.05 * 1.0e-3));
    }

    /// **The field can show the longest render there is.** A render
    /// wider than the box is clipped, and a clipped render reads as a
    /// different δ — so the width is measured against egui's own font
    /// metrics for the widest text
    /// `DisplayTolerance::RENDER_MM_MAX_CHARS` characters can spell out
    /// of the alphabet a render uses, rather than asserted in prose.
    ///
    /// The chrome sets no text styles of its own, so the headless
    /// context's metrics are the application's.
    #[test]
    fn the_field_shows_the_longest_render() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            for character in "0123456789.e-".chars() {
                let mut text: String =
                    std::iter::repeat_n(character, DisplayTolerance::RENDER_MM_MAX_CHARS).collect();
                let shown = egui::TextEdit::singleline(&mut text)
                    .desired_width(super::FIELD_WIDTH)
                    .show(ui);
                // `TextEdit`'s own horizontal margin, which its text
                // area does not get: `Margin::symmetric(4, 2)`.
                let room = shown.response.rect.width() - 8.0;
                let wanted = shown.galley.size().x;
                assert!(
                    wanted <= room,
                    "ten '{character}' need {wanted} points and the field offers {room}"
                );
            }
        });
        output.textures_delta.clear();
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
