---
id: tone-to-chrome-mapping-is-spelled-twice
kind: issue
title: The tone-to-chrome mapping is spelled at the toolbar and again at the feature row
status: closed
opened: 2026-09-19
closed: 2026-09-20
pr: 2915
branch: vnews/tone-row-badge
---



Filed by `tone-is-a-value-in-frame-and-a-comment-in-two-panes`'s lane,
which created the second site and could not merge the two: the first is
in `crates/viewer/src/app.rs`, which is CHROME's, VIEW's and VSEAM's.

**The tone is one value with two paints.** `app::draw_badge` maps a
badge's `frame::Tone` onto `RichText::weak()` and
`RichText::color(chrome(theme.unresolved))`; `pane::features`'s row
badge now reads `RowStatus::tone()` and makes the same two-arm mapping
again. What the row fix removed is the re-derivation of WHICH rows are
actionable; what it could not remove is the re-derivation of what an
actionable thing LOOKS like.

That second rule is the one CHROME's `chrome-weight-is-outside-the-
palette` is about — `ui.weak` is an egui default no palette can tune,
while `theme.unresolved` is a palette colour held to the marks check —
so a single `Tone -> RichText` door is also the place that row's first
decision would land.

**The shape:** one `pub(crate) fn` from `(&Theme, Tone)` to the text
style, called by both. Its natural home is beside `chrome()` in
`app.rs`, which is an announced crossing out of VNEWS, or beside
`Theme::unresolved` in `theme.rs`, which is out of fence in the other
direction. Either way it is not a VNEWS-only edit, which is why this is
a row rather than a residue of the unit that found it.

**What is NOT wrong here:** neither site invents a colour. `chrome()`
is the crate's one `Rgba8 -> Color32` door and both go through it; the
defect is that the two-arm MAPPING is written twice, so a change to
what `Advisory` looks like reaches one badge family and not the other.

## Closed at the fix pass that would otherwise have left it open
(2026-09-20)

Filed as residue, then taken in the same PR once the orchestrator ruled
that `app.rs` was not behind this lane's `frame.rs` fence. **The row was
answered by making the sentence true rather than by re-wording it**:
`app::draw_badge`'s comment said *"The tone's two spellings, in the one
place the mapping is made"*, and the diff that created this row made
that false.

`app::toned(text, theme, tone) -> egui::RichText` is that one place
now. Its readers are `app::draw_badge` for the toolbar badge family and
`pane::features::feature_row` for the tree's row badge — two families,
one rule, so what `Advisory` looks like changes in one place or
nowhere. The claim moved onto the function's own doc, where a reader
meets it.

**What this does NOT settle**, and why
`work/chrome/chrome-weight-is-outside-the-palette` still stands: the
mapping having one home makes that row's decision 1 — whether a
semantic distinction drawn in WEIGHT belongs inside the theme
contract — answerable in one edit, but does not answer it. `ui.weak` is
still an egui default no palette can tune, and the paint is still not
headlessly assertable.
