---
id: the-extrude-and-revolve-forms-take-no-variables
kind: issue
title: viewer: the extrude and revolve forms take only numbers: a distance or angle cannot be written over a variable
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [creating-a-thing-and-editing-it-are-two-implementations, the-add-profile-form-takes-numbers-and-no-formula]
---
Reported by Ev (chat, 2026-11-01): "extrude and revolve don't appear
to accept variables." Filed P1 at Ev's request.

## What happens

- **Extrude**: `extrude_ui` (`crates/viewer/src/pane/create.rs:1083`)
  holds the distance as a bare number (`unit_field` over
  `drafts.extrude_distance`, `create.rs:1095-1100`). It commits through
  `notation.length_literal` (`create.rs:1113`), which can only write a
  literal.
- **Revolve**: `revolve_tool_ui` (`create.rs:1133`) does the same for
  the axis point, heading and angle: `unit_field` at
  `create.rs:1162-1189`, committed through `point_literals` /
  `angle_literal` (`create.rs:1195-1199`).

Neither form has an expression door, so a variable can only be
attached afterwards from the Properties pane's slot row. (The
affordance test at `crates/viewer/src/pane/properties.rs:2336` drives an
extrude `Distance` slot by an expression, which suggests that route
exists.) Whether a person finds that route is part of the defect.

## Fix direction

Prefer the general fix in
`creating-a-thing-and-editing-it-are-two-implementations`: commit the
node with defaults and edit it in the slot rows, which already take
expressions. A form-local expression field would be a third copy of
the slot field. Survey the other seated tools (transform, pattern,
mirror, split, blend: `create.rs:1260-1652`) for the same
literal-only commit while here.
