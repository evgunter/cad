---
id: the-split-and-pattern-forms-take-no-values
kind: issue
title: The split and pattern create forms still hold value fields: a split takes a typed point and normal, a pattern its direction, spacing, axis and step
status: open
opened: 2026-11-03
priority: P1
cost: M
parent: creating-a-thing-and-editing-it-are-two-implementations
---


The last value fields in the seated create forms (`crates/viewer/src/pane/create.rs`). The ruling is Ev's on `[ev]` 6406, as written in `crates/viewer/README.md`, "A create gesture commits the node".

- **Split.** The form's `split_origin` and `split_normal` drafts (`crates/viewer/src/drafts.rs`) go. So do `DocSession::add_split`'s `read_off`/`frame_off` chain, which anchors the plane on the first edge and vertex by id (`crates/viewer/src/session.rs`), and the staged plane preview, with whatever only they use.
  - The gesture asks for the target and a plane: a picked planar face (`matetool::plane_of`, as the mirror does), or a plane already defined in the document.
  - With no plane picked, the button explains what it needs, as a tool with a missing read does.
  - Re-picking the plane goes through the operand (slot) door.
- **Pattern.** The direction, spacing, axis and step fields go. The insert writes default placement values, built on PLACING's `PatternDirection` (landed on main), and they are then edited on the pattern's edit surface: the placement editor's value rows, and the count row from #6414.
  - Decide whether the count field stays. It already has a row, so the create rule says it goes.
  - If the placement editor cannot yet edit a value the form used to take, that is a finding to report, not a reason to keep the field.
- `the-pattern-gesture-names-its-index` rides with this: the index is anonymous until a person names it (VR2).
(FORMS orchestrator)
