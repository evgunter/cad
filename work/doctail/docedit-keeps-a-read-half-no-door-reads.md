---
id: docedit-keeps-a-read-half-no-door-reads
kind: issue
title: DocEdit and the authored formula keep a serde read half that no door reads since the save format dropped its edit log
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found by `the-save-format-drops-its-edit-log`. Before that unit, the
save format's `edits` field was the one production reader of
`DocEdit`'s `serde::Deserialize` (`crates/editor-core/src/edit.rs`, the
derive on `DocEdit`) and so of the authored formula's wire read half
(`persist::wire::WireFormula`'s `rebuild`, its `Name`/`Fresh`/`Quantity`
leaves, and `persist::nesting`'s `Child` guard on that path). Now only
tests deserialize a `DocEdit`, among them
`edit_set_program::the_persisted_spelling_is_pinned_and_an_old_file_refuses_typed`,
whose doc still calls the edit's spelling "the format's compatibility
contract".

The write half stays live: the mint preimage serializes an edit's
statement (`crate::mint`). What is owed is a decision on the read half.
It can be removed along with the tests that pin it, or kept for a named
consumer (a history store, GUI-6, would be one), and then the doc that
calls it a format contract needs rewording. Until that decision, it is
code that no door reaches.
