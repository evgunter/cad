---
id: clear-picks-hover-text-is-invisible-while-disabled
kind: issue
title: Six gated controls carry their only words on on_hover_text, which shows nothing while they are disabled
status: closed
opened: 2026-09-19
refs: [a-disabled-control-says-why-in-four-shapes]
priority: P1
cost: E
closed: 2026-09-24
branch: vnews/gated-controls-say-why-while-disabled
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
`pane/properties.rs`'s parameter `range?` button (`:909`),
`widgets.rs:870` (`delete_button`'s
affordance hover, on a plain `ui.button`), `app.rs:228`, `:1375`,
`:1535`, `:1589`, `:1601`.

**What this sweep could not match**: a control disabled by an ancestor
`add_enabled_ui` rather than by its own call, and a hover attached
through a helper that takes the response by value. `widgets.rs:870` is
the only helper of that shape in the crate and its receiver is
ungated, so the second blind spot is empty today.

**The exemplar of the fix is in the tree**:
`pane/properties.rs`'s `range_button` (`:845-856`) branches on
whether the probe is refused — `Panel::probe_refusal`'s `Option` —
and hands `on_hover_text` to the live case and
`on_disabled_hover_text` to the other. Every one of the six can take
that shape; what the six need is the branch, and the value the branch
reads is whatever their own gate already computes.

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

## Fixed (VNEWS, 2026-09-24)

Each of the six now carries a sentence on each hook — `on_hover_text`
while it is live, `on_disabled_hover_text` while it is not — with the
live wording unchanged. None reads a `Refusal`: each is a draft gate,
and the words stay at the control.

| control | disabled when | what the hover says while disabled |
|---|---|---|
| `pane/create.rs`'s `clear_picks_button` | `count == 0` | *"no edge is picked, so there is nothing to clear"* |
| Revert, in `pane/profile.rs`'s `apply_and_revert` | `!moved` | *"nothing to revert: the numbers are the committed profile's"* |
| remove (`×`) | locked | *"remove this step"* and under it `forms::SHAPE_LOCKED` |
| move earlier (`⬆`) | locked; else `index == 0` | the action, and under it `SHAPE_LOCKED`, else *"it is already the first step"* |
| move later (`⬇`) | locked; else `index == last` | the action, and under it `SHAPE_LOCKED`, else *"it is already the last step"* |
| insert (`+`) | locked | *"insert a step after this one"* and under it `SHAPE_LOCKED` |

**Where the lock notice covers it.** For the `free` conjunct the
notice drawn above the list is the reason, so the four glyph controls
read that value (`SHAPE_LOCKED`) rather than paraphrasing it. What
the notice did not do was put the reason on the control the pointer is
on, and a glyph is a control's only label, so a disabled glyph also
names what it would have done. The lock is checked first: a lone row
in a locked list is stopped by both the lock and its index, and the
lock is the one reason that holds whichever row it is. `moved`, `index == 0` and
`index == last` had no sentence anywhere, and each has its own now.

**The exemplar, re-derived.** After #2961, `pane/properties.rs`'s
`ViewerBehavior::range_button` still has the shape this row cites: one
`add_enabled`, then a `match` on `probe_refusal` that hands the live
arm to `on_hover_text` and the refused arm to
`on_disabled_hover_text`. Its line numbers changed. Its words come from
a `Refusal`, so it is the shape these six copy and not their source.
The closer precedent for a draft gate is `pane/create.rs`'s
`all_edges_row`, which already carries its own literal on each hook.
Revert and Clear picks copy it. The four step glyphs share one private
`step_control`, because they build the same disabled sentence (action,
then reason). These are near-copies of one another and of
`app::refusable_button`, and consolidating them is filed as
`a-gated-button-with-a-reason-is-spelled-five-ways`.

**Apply, beside Revert.** Apply is disabled on the same `moved`. Its
comment used to defend silence ("the button says so by being
unavailable"), and that is the argument this row rejects. So Apply now
says *"nothing to apply: the numbers are the committed profile's"*.
Both buttons read one `UNTOUCHED` clause. Apply's `!refused` conjunct
adds no hover of its own, because `preview_verdict` already draws that
sentence under the step list.

**Held by headless rows.** `crate::pane::headless` gains
`painted_while_hovering`, which rests the pointer on a painted widget
past egui's tooltip delay and reads what the frame painted. Each
disabled sentence is asserted by its fixed text, and each live one
too. The end arrows are also asserted on a two-row list, where a
one-row gate would pass a one-row test. Clear picks, Apply and Revert
moved out of their methods into free functions (`clear_picks_button`,
`apply_and_revert`) so that a test can drive them.

**The sweep, re-run on `main` at `ebc22f34c`.** Every
`on_hover_text` and `on_hover_ui` in `crates/viewer/src` that is not
`on_disabled_hover_*` gives 17 lines, the same count as at the
census's base. The six above are the only ones on a gated receiver.
Of the rest, `pane/properties.rs`'s `range_button` (live arm) and
`pane/create.rs`'s `all_edges_row` (ready arm) already branch. The
others hang off `ui.button`, `ui.label`, `ui.weak`,
`ui.small_button`, or a response built ungated inside a helper
(`app.rs`'s `draw_badge`, `widgets.rs`'s `delete_button`).

**What that pattern cannot match, and the second pass at it.**
(1) A control disabled by an ancestor rather than by its own call.
There are now **seven** `add_enabled_ui` sites (`pane/profile.rs`'s
verb combo; `widgets.rs` ×6), not the census's six, and every one
still gates on `ShapeEdits::free()`. No `on_hover_text` sits inside
any of them, and `ui.disable()`, `set_enabled`, `UiBuilder` and
`interactable(false)` appear nowhere in the crate. (2) A hover
attached through a helper that takes a response. `draw_badge` and
`delete_button` build their own receivers ungated, and none of their
callers is inside an `add_enabled_ui`. (3) Tooltips reached through
`Tooltip::` or `show_tooltip*` directly: none. Nothing in the pass
turned up a seventh member.
