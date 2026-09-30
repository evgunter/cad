---
id: a-tools-commit-button-spells-its-name-a-second-time
kind: issue
title: a tool's commit button spells the tool's name a second time, beside the ToolKind that already carries it
status: open
opened: 2026-09-30
priority: P3
cost: E
---


## Finding

Found by AUTH-12's sweep, which routed the nine activation buttons
through `ToolKind::button` (one spelling of each tool's name, in
`ToolKind::label`). The second pass of that sweep read every button
literal in `crates/viewer/src`. It found the same defect one button
further into each panel: every tool's COMMIT button spells the tool's
name by hand, a second time.

- `ViewerBehavior::tool_commit_row` (`pane/create.rs`) takes `label:
  &str` and `kind: ToolKind` side by side. Every caller passes a label
  that is `"Commit "` plus the kind's name without its `" tool"`:
  `"Commit revolve"` (`revolve_tool_ui`), `"Commit boolean"`,
  `"Commit split"`, `"Commit transform"`, `"Commit pattern"`,
  `"Commit projection"` (`projection_tool_ui`, for `ToolKind::Part`)
  and `"Commit duplicate"`.
- `mate_tool_ui` and `blend_tool_ui` paint their own commit rows, with
  `"Commit mate"` and `"Commit blend"` as literals.

Renaming a tool in `ToolKind::label` renames its activation button and
its sentences, and leaves its commit button saying the old name.

## Why it was not fixed in AUTH-12

The commit words are the name *without* `" tool"`, and `label()`
returns the name *with* it. So one spelling needs a choice about how
the name is split: a bare noun that `label()` composes from, or a
commit form derived from `label()`. A second name function beside
`label()` is the duplication AUTH-12 was told not to mint. Choosing
between those two is the work. It is `E` once the choice is made:
nine call sites, and `tool_commit_row` loses its `label` parameter.
