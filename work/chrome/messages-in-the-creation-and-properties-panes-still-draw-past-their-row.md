---
id: messages-in-the-creation-and-properties-panes-still-draw-past-their-row
kind: issue
title: viewer: eleven refusal and fault sentences in the creation and properties panes are still drawn by the layout's rule, not the message's
status: open
opened: 2026-09-22
---


`crates/viewer/src/widgets.rs`'s `message` and `message_link` are the
one home for *a sentence drawn inside the region it is drawn in*, added
by the layout half of `error-and-check-text-overflows-its-region`. The
sites below were NOT routed through it, for one reason only:
`pane/create.rs` and `pane/properties.rs` were live in three other
lanes' open PRs (3052, 2960, 2961) in the wave that added it. Nothing
about them is harder than the sites that were converted.

## Drawn in a non-wrapping horizontal row — these run past the edge

egui gives a label in `ui.horizontal` `TextWrapMode::Extend`, which
lays the text out at infinite width, so a sentence longer than the pane
is drawn past its right-hand edge rather than wrapped or shortened.
Four sites, each a refusal wording beside a control:

- `pane/create.rs`, `picker`'s empty arm — *"none in this document —
  add a frame datum first"*, in the row that holds the combo.
- `pane/properties.rs`, `add_param_ui`'s existing-name arm —
  `Refusal::exists_wording`, beside its `edit <name>` link.
- `pane/properties.rs`, `slot_notes_ui` — `Refusal::affordance`, beside
  one `edit <name>` link per parameter. The longest of the four: its
  wording grows with the expression's parameter list.
- `pane/properties.rs`, the bounds row — `BoundsReading::wording`,
  beside the `range?` button.

## Drawn at the top of a pane function — correct today, by the caller

These are `ui.weak`/`ui.colored_label`/`ui.label` calls sitting
directly in a `*_ui` body, which its pane calls from a top-down layout,
where egui's default IS a wrap at the pane's width. They are not drawn
wrongly; what they lack is any reason they could not be. The rule is
the caller's layout rather than the message's kind, and the two callers
of one of them are two different forms:

- `pane/create.rs`: `add_part_ui`'s `refusal.to_string()`,
  `add_datum_ui`'s `fault.to_string()`, `add_profile_ui`'s `reason`,
  and `mate_tool_ui`'s two pick prompts.
- `pane/properties.rs`: `add_param_ui`'s `Refusal::offer_wording`,
  `entity_standing_ui`'s caveat, `instance_ui`'s `fault.to_string()`,
  and `slot_value_ui`'s `{error}`.

`pane/profile.rs`'s `preview_verdict` was in exactly this class and was
converted, because it is a free function with two callers in two
different forms — the argument applies to the rest unchanged.

## What a fix owes

`widgets::message_tests` measures the rule (a message's lines stay
inside the region, and every line begins under the first) against a
region laid out in a real `egui::Context`; nothing measures it at these
call sites, because `create_ui` and the properties rows hang off
`ViewerBehavior` and `crate::pane::headless` cannot drive a pane method
(`headless-egui-harness-spelled-five-times` carries that limit). The
four sites in the first list are free functions or could be made ones,
which is the cheap route to a row that can go red.
