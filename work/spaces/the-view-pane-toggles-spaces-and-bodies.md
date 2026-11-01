---
id: the-view-pane-toggles-spaces-and-bodies
kind: unit
title: the View pane lists the shown spaces and every body, each with a show/hide toggle
status: dispatched
opened: 2026-11-01
priority: P1
cost: M
branch: spaces/view-toggles
refs: [6209]
---


## Spec

`crates/viewer/README.md`, *The picture draws bodies*: the View pane
lists the shown spaces, the world first, and every body of the document
under its space, each with a toggle; the rule decides only the default,
so a body it leaves out can be shown and any drawn one hidden (Ev, PR
6209). Display state, never persisted (G3). Per-instance hide
(`hide_toggle` in the Properties pane) folds into this.

**Done when.** Rows: hide the world, hide a space, show an operand a
boolean took over, hide a drawn body; none of it reaches the document.
Carries `view-pane-cannot-hide-a-shown-space`.

Review tier: single, STYLE.
