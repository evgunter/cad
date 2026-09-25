---
id: a-gated-button-with-a-reason-is-spelled-five-ways
kind: issue
title: A button gated on a reason, with the reason on its disabled hover, is spelled five ways
status: open
opened: 2026-09-25
priority: P3
cost: D
refs: [clear-picks-hover-text-is-invisible-while-disabled, a-disabled-control-says-why-in-four-shapes]
---

Found on 2026-09-25 by the style review of #3216, the fix for
`clear-picks-hover-text-is-invisible-while-disabled`. The review ran on
`main` after #2960 landed.

## The shape

The pattern is `ui.add_enabled(blocked.is_none(), Button::new(label))`,
then a branch. The live arm goes to `on_hover_text`, or to nothing. The
blocked arm goes to `on_disabled_hover_text(reason)`. The spellings
vary in three ways:

- **The reason's type.** It is a `&Refusal`, a `&str`, or a `bool`
  with the words fixed inside the function.
- **Whether there is a live hover.**
- **Whether the disabled hover also names the action.** A glyph needs
  this, because the glyph is its only label.

## The population

| spelling | reason | live hover |
|---|---|---|
| `crate::app`'s `refusable_button`. Four callers: the cancel doors, Undo/Redo, New-document Create, and `pane/create.rs`'s `part_entry` | `Option<&Refusal>` | none |
| `pane/properties.rs`'s `ViewerBehavior::range_button` | `Option<Refusal>`, from `probe_refusal` | `PROBE_HOVER` |
| `pane/profile.rs`'s `step_control`, four glyphs per row | `Option<&str>` | the action |
| `pane/profile.rs`'s `apply_and_revert` and `pane/create.rs`'s `clear_picks_button`, which are the same code with different strings | a `bool` gate, with the literal inside | Revert and Clear picks have one; Apply has none |
| `pane/create.rs`'s `all_edges_row`, as two separate branches | a literal | yes |

Some `on_disabled_hover_text` calls are left out because they have a
gate but no branch:

- `pane/create.rs`'s Extrude, on its `None` arm, which is always
  disabled.
- `app.rs`'s Open… and Save As…, which carry
  `platform::NO_CHOOSER_BACKEND`.
- `pane/properties.rs`'s parameter Create.

The two combo-row pickers are also left out. They are
`two-pickers-spell-one-not-well-typed-sentence-twice`'s.

## What the consolidation has to decide

The skeleton is the same in every row. What differs is the reason's
type. A `Refusal` renders itself, and a draft gate's literal is a
`&str`. So one door taking `blocked: Option<impl Display>`, plus an
optional live hover, would serve every row.

The census's distinction should stay visible where the door is called:
a `Refusal` is read from the operation, and a literal is written at the
control. The door should not hide which one a caller is doing.

## Related: the headless frame drive

`crate::pane::headless`'s private `frame` is that module's one drive:
`run_ui` followed by `textures_delta.clear()`. Outside that module, the
same pair is inlined at 10 test sites:

- `widgets.rs`: 4
- `pane/view.rs`: 2
- `pane/viewport.rs`: 1
- `pane/profile.rs`: 1
- `app.rs`: 2

Whether those sites should call `frame` is the same kind of question,
and it can land in the same unit.
