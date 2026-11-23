---
id: selecting-an-index-variable-shows-no-row
kind: issue
title: selecting an index variable shows no row: its count has no field, and the variables list omits it
status: open
opened: 2026-11-01
---

## Finding

Found by the sweep of `spaces/pattern-value-rows`. A slot that reads a
pattern's index (`scalar(k) * (0.07 m * 1.0)`, the pattern tool's slide)
offers an `edit k` door under its row (`slot_notes`,
`crates/viewer/src/pane/properties.rs`). Clicking it selects the index
(`Selection::Variable(k)`), and the variable branch of the Properties
pane draws a row only for what `props::variable_rows` (named FREE
variables) or `props::defined_rows` (named DEFINED variables) list. An
index (`VarDef::Index`) is neither, so the pane draws nothing for it:
no count, no field, while the session already has the door a count
field would use (`set_variable_text` routes text typed over an index
through `props::index_count_edit`). The variables list at the bottom
of the pane omits indices for the same reason.

Done when a selected index shows its count in a field whose text door
is `SessionOp::SetVariableText`, and the variables list names it.
