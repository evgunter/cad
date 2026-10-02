---
id: the-create-pane-has-no-extrude-side
kind: issue
title: The extrude form has no side control, so a ruled depth-and-side extrude cannot be authored from the GUI
status: open
opened: 2026-10-01
priority: P1
cost: M
blocked_on: [extrude-distance-is-a-depth-and-a-side]
---


The viewer half of Ev's ruling on #3551 (2026-10-01): an extrude's distance is a positive depth with a structural `side`. It is blocked on EDIT's `extrude-distance-is-a-depth-and-a-side`, which builds the node and the kernel door.

**What to build here:**
- a `side` control in the extrude form (`pane/create.rs`, `extrude_ui`);
- `SessionOp::AddExtrude` carrying it;
- the property panel's structural-parameter row for an existing extrude's `side`.

The negative-depth refusal's recourse names `side`, and the form should make that recourse one click: the form's own control, not a typed sentence the author must act on.

The range probe needs no change: the floor becomes a half-line every seed brackets.
