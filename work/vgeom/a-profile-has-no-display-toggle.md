---
id: a-profile-has-no-display-toggle
kind: issue
title: viewer: a profile's display cannot be toggled in the View pane the way a body's can
status: closed
opened: 2026-11-22
priority: P1
cost: M
closed: 2026-11-25
pr: 7771
---

**Ev reported this** (in chat, 2026-11-22): "profiles should be able to have their display toggled in the view pane, like actual parts".

The View pane's toggles (`crates/viewer/src/pane/view.rs:94-136`, `toggles_ui`) come from `DisplayState::toggles` (`crates/viewer/src/display.rs:1337-1368`), which skips every node that is not `builds_a_body` (`display.rs:1347`). `set_body_on` refuses a non-body through `body_check` (`display.rs:1532-1538`), and the per-node toggle in Properties is hidden for a profile (`crates/viewer/src/pane/properties.rs:636`).

Every committed profile is drawn on every frame (`crates/viewer/src/pane/viewport.rs:290-304`, `:1442-1454`). That was the closed `work/vgeom/committed-profiles-are-not-drawn-in-the-viewport.md` (PR 2859), which made profiles display always, like the datums. Ev now wants them toggleable. The default of shown can stay; what is missing is the off switch, in the View pane and in Properties, and the pick index honouring it (`crates/viewer/src/pickindex.rs:1422`).

This pairs with `a-profile-has-no-free-move`. `pane/view.rs` and `display.rs` are shared with CHROME.

## Closed (2026-11-25)

PR 7771: a profile (and a datum) can be toggled in the View pane and in Properties, hides with the space it is drawn in, and is drawn and picked through one door (`DisplayView::outline`) under that space's free-move frame and the placement draft. Residue: `a-free-moved-outline-is-drawn-untinted-and-marked-at-its-first-drawing`, `an-edited-profile-in-a-moved-space-is-previewed-where-it-stood`, `a-copy-out-of-a-space-carries-every-outline-in-it`.
