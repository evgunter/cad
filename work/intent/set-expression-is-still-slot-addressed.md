---
id: set-expression-is-still-slot-addressed
kind: issue
title: SetExpression still edits a slot's expansion at an ExprPath; spec §1 and Q8 re-address it to a defined variable
status: open
opened: 2026-10-07
---

`docs/INTENT-LITERALS-SPEC.md` §1 ("Edits") gives the final shape as
`SetExpression { var: VarRef, path, formula }`, editing a defined
variable's definition, with the slot-addressed `ExprPath` kept as a
diagnosis address; §10 Q8 recommends it and §11 accepts every
recommendation. Neither PR C's list (§4) nor PR D's (§5) carries it,
and after D the arm is still `DocEdit::SetExpression { path: ExprPath,
expr: Formula }` (`crates/editor-core/src/edit.rs`), applied through
`Doc::expansion_along` to the slot's anonymous definition — so an edit
"at a path" re-addresses a slot, not a variable, and a named defined
variable's definition is edited only through `DefineVar`.

Found by the D lane checking §1's final shapes against the tree; not
D's to change (no §5 row, and the edit's wire spelling is a format
change of its own).
