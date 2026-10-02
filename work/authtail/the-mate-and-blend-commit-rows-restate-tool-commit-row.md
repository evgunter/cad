---
id: the-mate-and-blend-commit-rows-restate-tool-commit-row
kind: issue
title: the mate and blend commit rows restate tool_commit_row's commit, Cancel and close shape by hand
status: open
opened: 2026-09-30
priority: P4
cost: M
---


## Finding

Found by AUTH-12's fix-pass review (PR 3573). `tool_commit_row`
(`crates/viewer/src/pane/create.rs`) is documented as "the one place
[the combining tools'] two halves of the close rule live". It paints the
commit button (`kind.commit()`), queues the op or says the fault through
`kind.says`, and closes the tool on Cancel. Two panels repeat that shape
by hand:

- **`blend_commit_row`** paints `ToolKind::Blend.commit()`, pushes the
  op or says a `BlendError` or a length fault through
  `ToolKind::Blend.says`, and closes on Cancel. The same
  `let mut close` / `ui.horizontal` / `if close { self.tools.close() }`
  frame is written out again. What differs: a third button
  (`clear_picks_button`) between commit and Cancel, and the op is built
  from `self.tools.blend()` at the click rather than from a copy the
  panel took. `tool_commit_row`'s `op: impl FnOnce(&Drafts)` cannot
  borrow the tools while `self` is borrowed.
- **The mate panel's commit row** (in `mate_tool_ui`) paints
  `ToolKind::Mate.commit()` and says its refusals through
  `ToolKind::Mate.says`. Unlike the others, it closes the tool at its
  own click on `Ok` (`ToolKind::commits` answers `false` for `Mate`, so
  the application does not close it), and it reads the landed pair to
  build its proposal.

So the three rows share the button, the fault sentence and the Cancel
door, and differ in their close rule and in what the op is built from.
Folding them means either giving `tool_commit_row` a middle slot and a
close-on-commit flag, or stating why these two stay apart. Choosing
between those is the work, which is why the cost is M rather than E.
