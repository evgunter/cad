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
/// **What it still cannot reach is a pane METHOD**: `create_ui`,
/// `feature_row` and the rest hang off `ViewerBehavior`, which borrows
/// the whole application. A row a test must drive is therefore a free
/// function over the `Ui`, and the method's job is to call it.
#[cfg(test)]
pub(crate) mod headless {
    // Panicking is a test harness's failure mechanism, as it is a
    // test's (workspace lint note).
    #![allow(clippy::panic)]

    use eframe::egui;

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
    pub(crate) fn painted_after_clicking(
        target: &str,
        mut draw: impl FnMut(&mut egui::Ui),
    ) -> String {
        let ctx = egui::Context::default();
        let run = |input: egui::RawInput, draw: &mut dyn FnMut(&mut egui::Ui)| {
            let mut output = ctx.run_ui(input, |ui| draw(ui));
            let landed = landed_in(&output.shapes);
            let at = hit(&landed, target);
            let text: Vec<String> = landed.into_iter().map(|landed| landed.text).collect();
            output.textures_delta.clear();
            (text, at)
        };
        let (_, at) = run(egui::RawInput::default(), &mut draw);
        let at = at.unwrap_or_else(|| panic!("`{target}` was never painted"));
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

    /// **What one frame PAINTED and WHERE** — one entry per
    /// `Shape::Text` the frame emitted, in paint order.
    ///
    /// The one read this harness has. What a frame said and where it
    /// put it come off the same walk because they come off the same
    /// shape, and a layout defect shows up only in the second: a
    /// sentence drawn past the right edge of its pane and a sentence
    /// wrapped inside it paint the same string.
    pub(crate) struct Landed {
        /// The galley's whole text.
        pub(crate) text: String,
        /// **Where the LAYOUT put the galley**: `pos` plus
        /// `egui::Galley::size`, so the leading space egui indents a
        /// first row by is INSIDE it. That is the box a pointer has
        /// to be in for the widget to be hit, which is why [`hit`]
        /// centres on this one and not on a row.
        pub(crate) allocated: egui::Rect,
        /// **Where the READER's eye finds the glyphs**: one rect per
        /// row of the galley, each EXCLUDING that leading space, so
        /// `left()` is the row's first glyph rather than the indent
        /// in front of it.
        ///
        /// The two conventions differ by exactly that indent, and
        /// they differ on purpose — a row measuring where a sentence
        /// is READ must not count an indent as text, and a click must.
        pub(crate) rows: Vec<egui::Rect>,
    }

    /// [`Landed`] for every `Shape::Text` in a tree of shapes.
    ///
    /// **The module's one walker.** Everything else here is a read of
    /// what it answers, so a shape kind that starts nesting text is
    /// taught to one `match` rather than to three.
    pub(crate) fn landed_in(shapes: &[egui::epaint::ClippedShape]) -> Vec<Landed> {
        fn walk(shape: &egui::Shape, out: &mut Vec<Landed>) {
            match shape {
                egui::Shape::Text(text) => out.push(Landed {
                    text: text.galley.text().to_owned(),
                    allocated: egui::Rect::from_min_size(text.pos, text.galley.size()),
                    rows: text
                        .galley
                        .rows
                        .iter()
                        .map(|row| {
                            row.rect_without_leading_space()
                                .translate(text.pos.to_vec2())
                        })
                        .collect(),
                }),
                egui::Shape::Vec(inner) => {
                    for shape in inner {
                        walk(shape, out);
                    }
                }
                _ => {}
            }
        }
        let mut out = Vec::new();
        for clipped in shapes {
            walk(&clipped.shape, &mut out);
        }
        out
    }

    /// One headless frame of `draw`, and [`landed_in`] over what it
    /// painted.
    ///
    /// **The module's one drive.** The `textures_delta.clear()` is
    /// the reason it is one: no painter took the frame's font atlas,
    /// and `TexturesDelta` panics on drop until one does — a detail
    /// of epaint that no caller should have to remember.
    pub(crate) fn landed(draw: impl FnOnce(&mut egui::Ui)) -> Vec<Landed> {
        let ctx = egui::Context::default();
        let mut draw = Some(draw);
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            if let Some(draw) = draw.take() {
                draw(ui);
            }
        });
        let out = landed_in(&output.shapes);
        output.textures_delta.clear();
        out
    }

    /// Every string one pass of `draw` PAINTED, in paint order.
    pub(crate) fn painted(draw: impl FnOnce(&mut egui::Ui)) -> Vec<String> {
        landed(draw).into_iter().map(|landed| landed.text).collect()
    }

    /// The centre of the text `target`, where it was painted — the
    /// [`Landed::allocated`] box, because this is the position a
    /// synthesized click is aimed at.
    fn hit(landed: &[Landed], target: &str) -> Option<egui::Pos2> {
        landed
            .iter()
            .find(|landed| landed.text == target)
            .map(|landed| landed.allocated.center())
    }
}
