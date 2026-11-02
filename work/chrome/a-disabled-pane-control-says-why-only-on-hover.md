---
id: a-disabled-pane-control-says-why-only-on-hover
kind: issue
title: pane controls that are disabled say why only on hover
status: open
priority: P3
cost: E
opened: 2026-11-01
---

## Finding

Found by the forms-at-rest unit's sweep (`work/chrome/the-chrome-shows-every-form-at-once.md`, Ev 2026-10-31: the variables row's disabled Create "says why it is disabled only on hover"). That unit draws the add-variable form's reason under its row; the same shape — a disabled control in a pane whose reason is only `on_disabled_hover_text` — stands in these places, swept with `grep -rn on_disabled_hover_text crates/viewer/src` at the unit's merge base:

- `crates/viewer/src/pane/features.rs`, `feature_row_ui`'s instance `shown` checkbox: the hide refusal only on hover (the Properties pane's `hide_toggle` in `crates/viewer/src/pane/properties.rs` draws its fault under the toggle — one control drawn two ways).
- `crates/viewer/src/widgets.rs`, `delete_button`: the delete refusal only on hover.
- `crates/viewer/src/pane/create.rs`, `part_entry`'s pick button (through `app::refusable_button`), the blend tool's `Select all edges` and commit, and `clear_picks_button`.
- `crates/viewer/src/pane/profile.rs`: the step rows' verb options and glyph buttons, and the editor's Apply and Revert.
- `crates/viewer/src/pane/properties.rs`: the slot unit picker, the naming field's `Name`, and the range button.
- `crates/viewer/src/widgets.rs`, the path-target options (`sketch::not_well_typed`).

Not in scope: the toolbar's Undo/Redo and Open…/Save/Save As… (`crates/viewer/README.md` rules on those). Decide per control whether the reason belongs under it, as the add-variable form and `hide_toggle` now do.

