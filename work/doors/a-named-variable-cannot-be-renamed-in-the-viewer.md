---
id: a-named-variable-cannot-be-renamed-in-the-viewer
kind: issue
title: viewer: a variable that already has a name cannot be renamed, though the op and the id-keyed model support it
status: open
opened: 2026-11-22
priority: P1
cost: E
---

**Ev reported this** (in chat, 2026-11-22). Ev found no way to rename a variable, and added that if things are keyed on its name, perhaps they should not be.

They are not. Readers hold a `VarId` (`crates/editor-core/src/formula.rs:127`, `expr.rs:643`, `:1014`, `operand.rs:38`). The name exists only in the authored form, `AuthoredLeaf::Name(VarName)` (`expr.rs:566-569`), which the edit door resolves to the id. `DocEdit::RenameVar` (`crates/editor-core/src/edit.rs:425-430`) "gives or clears its name and moves no reader" (`edit.rs:64-66`).

The viewer has `SessionOp::RenameVar` (`crates/viewer/src/session/op.rs:259-272`, wired at `crates/viewer/src/session.rs:2224`). Its only emitter is `name_field_ui` (`crates/viewer/src/pane/properties.rs:1215-1272`), reached from the naming button for unnamed variables (`properties.rs:1193-1199`), and it closes once the variable has a name (`:1232`). The selected-variable panel (`properties.rs:104-189`) and the variables list (`:191-217`) offer no rename.

The fix: an editable name on the selected-variable panel (and on its row, if `work/chrome/a-variable-is-edited-in-a-separate-widget-not-on-its-own-row.md` lands first) that emits `RenameVar`. A rename to a name already taken is refused with the kernel's own refusal.
