---
id: app-rs-gained-a-toned-door-from-a-vnews-unit
kind: issue
title: app.rs gained pub(crate) toned from a VNEWS unit, and its two callers are in two programs
status: open
opened: 2026-09-20
priority: P1
cost: E
---



**Notice, not a defect.** Filed so VSEAM meets this in a row rather
than in a merge: VSEAM has rows in flight on `crates/viewer/src/app.rs`
and a new function appeared in it from another program's unit.

Landed by `vnews`'s `tone-is-a-value-in-frame-and-a-comment-in-two-
panes` (PR 2915). The crossing was authorised by the VNEWS
orchestrator, who has since recorded that the authorisation covered
PROSE and did not by itself license a new door in this file — hence
this row.

## What is there now

`app::toned` (`crates/viewer/src/app.rs`, ~`:210`):

```rust
pub(crate) fn toned(text: impl Into<String>, theme: &Theme, tone: frame::Tone) -> egui::RichText
```

It is the one place the `frame::Tone` -> chrome mapping is made:
`Tone::Advisory` -> `RichText::weak()`, `Tone::Actionable` ->
`RichText::color(chrome(theme.unresolved))`.

**Two callers, in two programs' territory:**

- `app::draw_badge` (~`:231`) — the toolbar badge family's single
  draw, which previously spelled the two arms inline. **This is
  VSEAM's ground.**
- `pane::features::feature_row` (~`:89`) — the feature tree's row
  badge, reading `tree::RowStatus::tone()`. **VNEWS's ground.**

Five lines came out of `draw_badge` and no toolbar behaviour changed.

## Why it was put here rather than in `frame` or `theme`

`frame` is a vocabulary module and names no toolkit type, so it cannot
return an `egui::RichText`; `theme.rs` is the same and is out of both
lanes' fences besides. `chrome()` — the crate's one `Rgba8 ->
Color32` door — is already in `app.rs`, so this is where a mapping onto
toolkit text can live at all.

## The live consequence, which is the part worth a row

**`toned` is `pub(crate)`, so `crates/viewer/tests/*` cannot reach
it.** That is what keeps the paint un-assertable from an integration
row, and it is the remaining half of
`work/chrome/chrome-weight-is-outside-the-palette`'s decision 2 (now
`open`): the policy half is a pure function asserted headlessly
(`tree::RowStatus::tone`), and the paint half is a pure function that
no test outside the crate can call. Widening the visibility is a
decision about this file and therefore VSEAM's to make, alongside
CHROME's first decision about whether a distinction drawn in WEIGHT
belongs in the theme contract at all.

A VSEAM lane restructuring `app.rs` should know the function has a
caller outside this file and outside this program.
