---
id: a-tools-commit-button-spells-its-name-a-second-time
kind: issue
title: a tool's commit button spells the tool's name a second time, beside the ToolKind that already carries it
status: closed
opened: 2026-09-30
priority: P3
cost: E
closed: 2026-09-30
pr: 3573
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

## Built (AUTH-12's fix pass, PR 3573)

`ToolKind::label` had exactly two readers, `says` and `button`, and
both `format!`. So the match now holds the bare noun ("mate", …,
"projection"), and each spelling is a composer over it: `says` adds
`" tool: "`, `button` capitalises and adds `" tool…"`, and the new
`ToolKind::commit` prefixes `"Commit "`. `tool_commit_row` lost its
`label` parameter and reads `kind.commit()`. The mate and blend commit
buttons read `ToolKind::Mate.commit()` and `ToolKind::Blend.commit()`.
The painted text does not change.

What this row does not cover, and what is still open: `blend_commit_row`
and the mate panel's commit row repeat `tool_commit_row`'s
commit/Cancel/close shape by hand. That is
`work/author/the-mate-and-blend-commit-rows-restate-tool-commit-row`.
