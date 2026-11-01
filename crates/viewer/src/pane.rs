//! **The pane bodies**: one module per docked pane, each holding the
//! `*_ui` functions that draw it.
//!
//! Each is part of the `app` driver rather than a vocabulary — they
//! name `egui` and they hold `impl ViewerBehavior`. What they do not
//! do is mutate the session: a pane reads it as a value and pushes
//! [`crate::session::SessionOp`]s into a queue the application drains
//! after the layout has been walked (`crate::app`'s header states the
//! rule).
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

pub mod create;
pub mod features;
pub mod profile;
pub mod properties;
pub mod view;
pub mod viewport;

/// **Reading back what a pane PAINTED** — a real `egui::Context` laid
/// out with no window and no renderer, and the text off the frame's
/// shapes.
///
/// Driving a pane function headlessly is not new here: `pane::profile`
/// and `pane::viewport` both run one against a real context already.
/// What this adds is the read side. A drive that inspects the state a
/// widget left behind cannot tell a pane that composed the right
/// sentence from one that composed it and drew something else, and
/// that gap is where a labelling helper gets to be correct and dead at
/// the same time.
///
/// **A pane METHOD is reached through the whole app, not through
/// this module**: `create_ui`, `properties_ui` and the rest hang off
/// `ViewerBehavior`, which borrows the whole application, so a row that
/// drives one runs real frames of `ViewerApp` in
/// `app::properties_pane_tests` (`app_frame`, `Driven`) and reads what
/// they painted with [`headless::landed_in`]. What this module drives
/// is a free function over the `Ui`, which is the cheaper row where a
/// method's work can be one.
#[cfg(test)]
pub(crate) mod headless {
    // Panicking is a test harness's failure mechanism, as it is a
    // test's (workspace lint note).
    #![allow(clippy::panic)]

    use eframe::egui;

    use crate::widgets::message_floor;

    /// Rows are placed at whole pixels, so two readings of one edge
    /// can differ by less than one.
    pub(crate) const SLACK: f32 = 1.0;

    /// Everything one pass of `draw` painted, joined by newlines.
    pub(crate) fn painted_text(draw: impl FnOnce(&mut egui::Ui)) -> String {
        painted(draw).join("\n")
    }

    /// Everything `draw` paints once the widget carrying the text
    /// `target` has been CLICKED — for the state a widget only enters
    /// on interaction, such as an open combo's list.
    ///
    /// Three passes on ONE context, so egui's memory carries between
    /// them: one to lay out and find where `target` was painted, one
    /// that clicks its centre, and one that reads what the click
    /// opened. The answer is what the last two painted.
    ///
    /// Panics when `target` was not painted at all — a click at a
    /// guessed position would otherwise read as a widget that did not
    /// open.
    pub(crate) fn painted_after_clicking(target: &str, draw: impl FnMut(&mut egui::Ui)) -> String {
        painted_after_clicking_nth(target, 0, draw)
    }

    /// [`painted_after_clicking`] on the `nth` (zero-based) painting of
    /// `target` — for a control every row of a list carries.
    ///
    /// Panics when `target` was painted fewer than `nth + 1` times.
    pub(crate) fn painted_after_clicking_nth(
        target: &str,
        nth: usize,
        mut draw: impl FnMut(&mut egui::Ui),
    ) -> String {
        let ctx = egui::Context::default();
        let run = |input: egui::RawInput, draw: &mut dyn FnMut(&mut egui::Ui)| {
            let landed = frame(&ctx, input, draw);
            let at = hit(&landed, target, nth);
            let text: Vec<String> = landed.into_iter().map(|landed| landed.text).collect();
            (text, at)
        };
        let (_, at) = run(egui::RawInput::default(), &mut draw);
        let at =
            at.unwrap_or_else(|| panic!("`{target}` was painted fewer than {} times", nth + 1));
        let click = egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                },
            ],
            ..Default::default()
        };
        let (clicked, _) = run(click, &mut draw);
        let (after, _) = run(egui::RawInput::default(), &mut draw);
        [clicked.join("\n"), after.join("\n")].join("\n")
    }

    /// Everything `draw` paints while the pointer RESTS on the `nth`
    /// (zero-based) painting of the text `target` — for what a widget
    /// says only on hover. egui draws that through two hooks, one read
    /// only while the widget takes input and one only while it does
    /// not, so a row asserting a control's words hovers it in the
    /// state it asserts about.
    ///
    /// Frames on ONE context, at a clock this drive sets: one to lay
    /// out and find where `target` was painted, one that moves the
    /// pointer there, and then frames with the pointer still until
    /// well past egui's tooltip delay — a tooltip's area spends the
    /// first frame it appears sizing itself, invisible. The answer is
    /// what the last frame painted.
    ///
    /// Panics when `target` was painted fewer than `nth + 1` times.
    pub(crate) fn painted_while_hovering(
        target: &str,
        nth: usize,
        mut draw: impl FnMut(&mut egui::Ui),
    ) -> String {
        let ctx = egui::Context::default();
        let laid_out = timed(&ctx, 0.0, Vec::new(), &mut draw);
        rest_on(&ctx, &laid_out, target, nth, &mut draw)
    }

    /// [`painted_while_hovering`] inside what clicking `opener` opens —
    /// for the words a choice in an open combo's list says only on
    /// hover. The click lands on the first painting of `opener`, and
    /// `target` is found in the frame after it, with the list open.
    ///
    /// Panics when `opener` was never painted, or `target` was
    /// painted fewer than `nth + 1` times once it was clicked.
    pub(crate) fn painted_while_hovering_opened(
        opener: &str,
        target: &str,
        nth: usize,
        mut draw: impl FnMut(&mut egui::Ui),
    ) -> String {
        let ctx = egui::Context::default();
        let laid_out = timed(&ctx, 0.0, Vec::new(), &mut draw);
        let at =
            hit(&laid_out, opener, 0).unwrap_or_else(|| panic!("`{opener}` was never painted"));
        let press = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        timed(
            &ctx,
            0.1,
            vec![egui::Event::PointerMoved(at), press(true), press(false)],
            &mut draw,
        );
        let opened = timed(&ctx, 0.2, Vec::new(), &mut draw);
        rest_on(&ctx, &opened, target, nth, &mut draw)
    }

    /// One frame of `draw` on `ctx` at the clock reading `seconds`.
    fn timed(
        ctx: &egui::Context,
        seconds: f64,
        events: Vec<egui::Event>,
        draw: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<Landed> {
        let input = egui::RawInput {
            time: Some(seconds),
            events,
            ..Default::default()
        };
        frame(ctx, input, draw)
    }

    /// The hover half of [`painted_while_hovering`]: move the pointer
    /// onto the `nth` painting of `target` in `laid_out`, then rest it
    /// there until well past egui's tooltip delay, and answer what the
    /// last frame painted.
    fn rest_on(
        ctx: &egui::Context,
        laid_out: &[Landed],
        target: &str,
        nth: usize,
        draw: &mut dyn FnMut(&mut egui::Ui),
    ) -> String {
        let delay = f64::from(ctx.global_style().interaction.tooltip_delay);
        let at = hit(laid_out, target, nth)
            .unwrap_or_else(|| panic!("`{target}` was painted fewer than {} times", nth + 1));
        timed(ctx, 1.0, vec![egui::Event::PointerMoved(at)], draw);
        let mut rested = Vec::new();
        for rest in 1..=3 {
            rested = timed(ctx, 1.0 + 2.0 * delay * f64::from(rest), Vec::new(), draw);
        }
        rested
            .into_iter()
            .map(|landed| landed.text)
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(crate) use crate::app::drive::{Landed, landed_in};

    /// **One frame of `draw` on `ctx`, fed `input`**, and [`landed_in`]
    /// over what it painted.
    ///
    /// **This module's one drive.** The `textures_delta.clear()` is
    /// the reason it is one: no painter took the frame's font atlas,
    /// and `TexturesDelta` panics on drop until one does — a detail
    /// of epaint that no caller should have to remember. Test rows
    /// elsewhere in the crate still spell the pair inline
    /// (`work/vnews/a-gated-button-with-a-reason-is-spelled-five-ways`).
    fn frame(
        ctx: &egui::Context,
        input: egui::RawInput,
        draw: &mut dyn FnMut(&mut egui::Ui),
    ) -> Vec<Landed> {
        let mut output = ctx.run_ui(input, |ui| draw(ui));
        let landed = landed_in(&output.shapes);
        output.textures_delta.clear();
        landed
    }

    /// One headless frame of `draw`, and what it painted.
    pub(crate) fn landed(draw: impl FnOnce(&mut egui::Ui)) -> Vec<Landed> {
        let mut draw = Some(draw);
        landed_after(1, |ui| {
            if let Some(draw) = draw.take() {
                draw(ui);
            }
        })
    }

    /// **The two voices a [`crate::frame::Tone`] is drawn in**, as the
    /// fixed colours a row holds [`Landed::ink`] against — never read
    /// back through `app::toned`, which is the mapping under test.
    #[derive(Clone, Copy, Debug)]
    pub(crate) struct Voices {
        /// egui's own weak text on the frame that was drawn: the
        /// `Advisory` voice.
        pub(crate) weak: egui::Color32,
        /// The actionable colour of the theme the draw was handed: the
        /// `Actionable` voice.
        pub(crate) actionable: egui::Color32,
    }

    /// One headless frame of `draw` handed `theme`, on a context set to
    /// that theme's polarity as the application sets it, and the
    /// [`Voices`] of the frame it drew.
    ///
    /// Panics when the frame was drawn in the other polarity, or when
    /// the two voices are one colour, since then no row could tell them
    /// apart.
    pub(crate) fn landed_voiced(
        theme: &crate::theme::Theme,
        draw: impl FnOnce(&mut egui::Ui, &crate::theme::Theme),
    ) -> (Vec<Landed>, Voices) {
        let ctx = egui::Context::default();
        crate::app::apply_polarity(&ctx, theme.polarity);
        let weak = core::cell::Cell::new(egui::Color32::PLACEHOLDER);
        let mut draw = Some(draw);
        let painted = frame(&ctx, egui::RawInput::default(), &mut |ui| {
            assert_eq!(
                ui.visuals().dark_mode,
                theme.polarity == crate::theme::Polarity::Dark,
                "{}: the frame is drawn in the theme's polarity",
                theme.name,
            );
            weak.set(ui.visuals().weak_text_color());
            if let Some(draw) = draw.take() {
                draw(ui, theme);
            }
        });
        let voices = Voices {
            weak: weak.get(),
            actionable: crate::app::chrome(theme.actionable),
        };
        assert_ne!(
            voices.weak, voices.actionable,
            "the two voices a tone is drawn in"
        );
        (painted, voices)
    }

    /// The entry in `painted` whose text starts with `opening` — for a
    /// sentence whose tail is another layer's payload.
    ///
    /// Panics when nothing painted opens with it.
    pub(crate) fn find_opening<'a>(painted: &'a [Landed], opening: &str) -> &'a Landed {
        painted
            .iter()
            .find(|landed| landed.text.starts_with(opening))
            .unwrap_or_else(|| {
                panic!(
                    "nothing painted opens with `{opening}`: {:?}",
                    painted.iter().map(|l| &l.text).collect::<Vec<_>>()
                )
            })
    }

    /// [`landed`] over the LAST of `passes` frames of `draw` on one
    /// context — for a container whose first frame paints nothing,
    /// which is what an `egui::Window` does on the frame it first
    /// appears (an invisible sizing pass).
    pub(crate) fn landed_after(passes: usize, mut draw: impl FnMut(&mut egui::Ui)) -> Vec<Landed> {
        let ctx = egui::Context::default();
        let mut out = Vec::new();
        for _ in 0..passes {
            out = frame(&ctx, egui::RawInput::default(), &mut draw);
        }
        out
    }

    /// **One frame of `draw` in a pane `width` points wide**: the
    /// pane's rect, and everything painted in it — the region a
    /// pane's `layout_tests` measure a sentence against.
    ///
    /// Panics when `width` is at or under [`message_floor`], where a row would read the
    /// floor rather than the region.
    pub(crate) fn drawn_in(
        width: f32,
        draw: impl FnOnce(&mut egui::Ui),
    ) -> (egui::Rect, Vec<Landed>) {
        let region = core::cell::Cell::new(egui::Rect::NOTHING);
        let painted = landed(|ui| {
            ui.allocate_ui(egui::vec2(width, 800.0), |ui| {
                region.set(ui.max_rect());
                let floor = message_floor(ui);
                assert!(
                    width > floor,
                    "a {width}-point pane is at or under the {floor}-point floor, \
                     so this row would read the floor rather than the region"
                );
                draw(ui);
            });
        });
        (region.get(), painted)
    }

    /// The entry in `painted` whose text is `text`.
    ///
    /// Panics when nothing painted it.
    pub(crate) fn find<'a>(painted: &'a [Landed], text: &str) -> &'a Landed {
        painted
            .iter()
            .find(|landed| landed.text == text)
            .unwrap_or_else(|| panic!("`{text}` was never painted"))
    }

    /// Every row of `landed` inside `region`'s right-hand edge.
    pub(crate) fn assert_inside(region: egui::Rect, landed: &Landed) {
        for row in &landed.rows {
            assert!(
                row.right() <= region.right() + SLACK,
                "`{}` ends {} points past a {}-point pane (rows {:?})",
                landed.text,
                row.right() - region.right(),
                region.width(),
                landed.rows
            );
        }
    }

    /// [`assert_inside`], and every row starting at the pane's left
    /// edge: a line of its own, with nothing beside it.
    pub(crate) fn assert_own_lines(region: egui::Rect, landed: &Landed) {
        assert_inside(region, landed);
        for row in &landed.rows {
            assert!(
                (row.left() - region.left()).abs() <= SLACK,
                "`{}` has a row starting {} points into the pane rather than \
                 at its edge (rows {:?})",
                landed.text,
                row.left() - region.left(),
                landed.rows
            );
        }
    }

    /// `below` starts under the last row of `above`.
    pub(crate) fn assert_under(above: &Landed, below: &Landed) {
        let bottom = above
            .rows
            .iter()
            .map(egui::Rect::bottom)
            .fold(f32::NEG_INFINITY, f32::max);
        let top = below
            .rows
            .iter()
            .map(egui::Rect::top)
            .fold(f32::INFINITY, f32::min);
        assert!(
            top >= bottom - SLACK,
            "`{}` starts above the bottom of `{}` — beside it, not under it \
             ({top} against {bottom})",
            below.text,
            above.text
        );
    }

    /// Every string one pass of `draw` PAINTED, in paint order.
    pub(crate) fn painted(draw: impl FnOnce(&mut egui::Ui)) -> Vec<String> {
        landed(draw).into_iter().map(|landed| landed.text).collect()
    }

    /// The centre of the `nth` (zero-based) painting of the text
    /// `target` — the [`Landed::allocated`] box, because this is the
    /// position a synthesized pointer is aimed at.
    fn hit(landed: &[Landed], target: &str, nth: usize) -> Option<egui::Pos2> {
        landed
            .iter()
            .filter(|landed| landed.text == target)
            .nth(nth)
            .map(|landed| landed.allocated.center())
    }
}
