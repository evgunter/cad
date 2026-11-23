---
id: message-resolves-its-text-style-differently-from-ui-label
kind: issue
title: viewer: wrapped_in_region passes TextStyle::Body where Label::layout_in_ui passes FontSelection::Default, which differ under a style override
status: closed
opened: 2026-09-22
closed: 2026-09-22
pr: 3089
priority: P3
cost: E
---


`crates/viewer/src/widgets.rs`'s `wrapped_in_region` lays its galley
out with

    text.into().into_galley(ui, Some(egui::TextWrapMode::Wrap),
                            ui.available_width(), egui::TextStyle::Body)

where `egui::Label::layout_in_ui` — the path every unconverted site
still takes — passes `egui::FontSelection::Default`.

**For a plain `&str` the two agree.** For the `egui::RichText` arm they
do not: `FontSelection::Default` resolves through
`egui::Style::override_font_id` and `egui::Style::override_text_style`
first, and `TextStyle::Body` skips both. So a context-wide style
override would move every `ui.label` in the chrome and leave every
`widgets::message` at Body.

## Why it is filed rather than dismissed

**It is latent, not impossible.** No override is set today, so nothing
differs. But this crate DOES write context-wide style — that is exactly
what `widgets::install_number_formatter` exists to do, in the same file
— so "nobody sets one" is a fact about today and not a property of the
code. The next context-wide style would split `message` from
`ui.label` silently: the two spellings would render the same sentence
in different fonts, and the difference would show up as a layout
question rather than as a style one.

The same applies one step over: `message` now passes its wrap mode
explicitly and therefore also overrides a future
`egui::Style::wrap_mode`. That one is deliberate and `message`'s doc
says so. This one is not stated anywhere.

## What a fix owes

Either pass `FontSelection::Default` so the two paths resolve the same
way, or say at `wrapped_in_region` why a message's text style is not
the chrome's. A row that goes red is cheap here: lay the same sentence
out through `message` and through `ui.label` in one headless frame with
an `override_text_style` set, and compare the galleys.

## Closed

Closed by PR 3089 (`chrome/message-floor`), on a premise that did not
hold. In egui 0.36.1 the two fallbacks do not differ under an override.
`WidgetText::into_galley_impl`'s `Text` arm resolves
`FontSelection::default().resolve_with_fallback(style, fallback)`, and
`RichText::into_text_and_format` reads `style.override_font_id`, then
the text's own style or `style.override_text_style`, before it touches
the fallback. So both arms consult both overrides ahead of whatever
fallback is passed. With no override, `FontSelection::Default` falls
back to Body. The two spellings are equal under every style.

`wrapped_in_region` now passes `FontSelection::Default` anyway, so
that a message and `ui.label` make the same call rather than two calls
that agree, and its doc says why the font is the chrome's while the
wrap mode is the message's. `message_floor` measures in the same
resolved font. The row the finding proposed was written and then
deleted: it could not go red against the old code (proved by reverting
to `TextStyle::Body`, which stayed green), so it was documentation.
