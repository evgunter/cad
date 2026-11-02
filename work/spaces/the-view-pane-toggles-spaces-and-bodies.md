---
id: the-view-pane-toggles-spaces-and-bodies
kind: unit
title: the View pane lists the shown spaces and every body, each with a show/hide toggle
status: closed
opened: 2026-11-01
priority: P1
cost: M
branch: spaces/view-toggles
refs: [6209]
pr: 6270
closed: 2026-11-02
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

## Closed

PR 6270. The View pane lists the world, then every space holding a body,
each body under its space with a toggle; one override map
(`DisplayState::toggled`, node → on/off) relative to the default picture,
decided against what the toggle reads now; the picture key carries the
shown set; a dropped show is reported; the tree and Properties checkboxes
read the same toggles. STYLE review: one correctness MINOR (a shown `⊔`'s
piece toggled silently) and the two-set storage, fixed. Filed:
`the-display-budget-does-not-price-shown-bodies`,
`a-family-member-read-alone-has-no-toggle-of-its-own`,
`whether-a-body-only-shown-is-tinted-outside-the-product`,
`free-move-refuses-a-body-the-author-showed`.
