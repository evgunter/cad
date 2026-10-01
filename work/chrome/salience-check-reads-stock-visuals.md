---
id: salience-check-reads-stock-visuals
kind: issue
title: The actionable-salience check reads egui's stock Visuals, not the viewer's installed style
status: open
opened: 2026-10-01
priority: P4
cost: E
refs: [chrome-weight-is-outside-the-palette, 3634]
---


`tests/theme.rs`'s
`the_actionable_voice_is_told_from_the_panel_and_both_text_voices`
measures `Theme::actionable` against the panel, plain text and weak
text. It takes those from `cvd::chrome_voices`, which builds
`egui::Visuals::light()` / `dark()`, egui's stock per-polarity style.
The viewer draws with whatever `app::apply_polarity` installs on the
context. Today that is the same stock style, because `set_theme` leaves
egui's per-theme visuals intact.

Suppose the viewer starts tuning its visuals: a different
`panel_fill`, `override_text_color`, `weak_text_alpha` or
`weak_text_color`. The check would keep measuring the stock colours
and pass, and the claim in `crates/viewer/GUI-DESIGN.md`, Colour (G5),
would go unchecked for the colours actually drawn.

Fix: have the check read the visuals the viewer installs, through one
`app` function that both the application and the test call (for
example, a `pub` `Visuals` for a `Polarity`), instead of egui's
constructors.
