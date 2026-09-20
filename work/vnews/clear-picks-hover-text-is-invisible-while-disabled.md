---
id: clear-picks-hover-text-is-invisible-while-disabled
kind: issue
title: Six gated controls carry their only words on on_hover_text, which shows nothing while they are disabled
status: open
opened: 2026-09-19
refs: [a-disabled-control-says-why-in-four-shapes]
---

Found by the census in `a-disabled-control-says-why-in-four-shapes`, at
merge base `2654cc111417da806d9786c40136106469096fec`. Disclosed there
as one of the census's blind spots, with *"Clear picks"* named as the
instance; **swept when the row was written, and the blind spot holds
six members, not one.**

## The shape

`egui` shows `on_hover_text` only while the widget takes input. A
control that is sometimes disabled and carries its sentence there is
mute in exactly the state a sentence is for. `pane/create.rs`'s "Clear
picks" (`:1059-1060`):

```
ui.add_enabled(count > 0, egui::Button::new("Clear picks"))
    .on_hover_text("drop every picked edge and start on any body")
```

## The population — every `on_hover_text` on an `add_enabled` control

Swept as `on_hover_text` minus `on_disabled_hover_text` over
`crates/viewer/src` (17 lines), then each read at the line for whether
the receiver is gated. **Six are:**

| site | gate | the words that vanish |
|---|---|---|
| `pane/create.rs:1059-1060` | `count > 0` | *"drop every picked edge and start on any body"* |
| `pane/profile.rs:114-115` | `moved` | *"put the numbers back to the committed profile's"* |
| `pane/profile.rs:278-279` | `free` | *"remove this step"* |
| `pane/profile.rs:285-286` | `free && index > 0` | *"move this step earlier"* |
| `pane/profile.rs:292-293` | `free && index < last` | *"move this step later"* |
| `pane/profile.rs:313-314` | `free` | *"insert a step after this one"* |

The other eleven sit on ungated receivers — `ui.button`, `ui.label`,
`ui.weak` — and are not members: `pane/create.rs:241`, `:315`, `:996`,
`pane/properties.rs:806`, `widgets.rs:870` (`delete_button`'s
affordance hover, on a plain `ui.button`), `app.rs:228`, `:1375`,
`:1535`, `:1589`, `:1601`.

**What this sweep could not match**: a control disabled by an ancestor
`add_enabled_ui` rather than by its own call, and a hover attached
through a helper that takes the response by value. `widgets.rs:870` is
the only helper of that shape in the crate and its receiver is
ungated, so the second blind spot is empty today.

**The exemplar of the fix is in the tree**:
`pane/properties.rs`'s `range_button` (`:767-774`) branches on its own
`offered` and hands `on_hover_text` to the live case and
`on_disabled_hover_text` to the other. Every one of the six can take
that shape.

## Why this is not the census's own class

By the census's rule these are all **draft gates**: nothing to clear,
nothing to revert, nothing to reorder, and a locked editor with no edit
door behind it. No `SessionOp` is formed and no refusal exists to be a
second copy of, so a literal at the control is correct and the six
literals are true. The defect is one level down — the literal is on the
wrong hook, so the disabled state has no words rather than wrong ones.

That makes it a separate class, and the class is the interesting part:
a sweep over `on_disabled_hover_text` cannot see it, and a sweep over
`add_enabled` sees the control but not which hook its text rides on.

**The five `pane/profile.rs` members are partly covered already**:
`forms::SHAPE_LOCKED`, drawn once above the step list
(`pane/profile.rs:72`), says why the `free` conjunct is false. It says
nothing about `moved`, about `index > 0` or about `index < last`, and
it is a sentence about the editor rather than about the control the
reader's pointer is on.

## Home

VNEWS's for `pane/create.rs`, which is in this program's `paths`.
`pane/profile.rs` is claimed by no VIEW successor
(`work/view/viewer-src-files-no-successor-claims`); the five members
there are the same defect and a lane that takes them announces the
crossing.
