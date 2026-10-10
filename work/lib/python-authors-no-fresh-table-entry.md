---
id: python-authors-no-fresh-table-entry
kind: issue
title: Python authors no fresh-table entry, so a value shared within one edit takes a declare first
status: open
priority: P3
cost: M
opened: 2026-10-09
refs: [a-shared-variable-is-named-at-the-doors, python-edit-error-carries-no-handle-for-an-unnamed-variable]
---

An edit's fresh table (`DocEdit::InsertNode`'s and the other arms' `fresh: Vec<FreshEntry>`, `crates/editor-core/src/var.rs`, `FreshEntry`) is how one edit mints a variable and reads it from several places. Since FORK-7 an entry carries a name, so one insert can mint a named variable that two of its slots share.

Python builds no fresh table. Every `DocEdit` constructor in `crates/pncad-py/src/py/doc.rs` passes `fresh: Vec::new()`, `Formula` has no `fresh` leaf, and `test_binding_census.py` lists `FreshEntry` as different-shape. A Python caller who wants two slots of a new node to share a new value has to declare it first (`DocEdit.declare_var`) and read it by name: two edits where the Rust façade spends one, and two undo steps in a session that records Python's edits.

Either bind a fresh table (a `Formula.fresh(i, dim)` leaf and a `fresh=` argument carrying `(name, decl)` entries), or rule that Python declares first and say so in the stub. This is the sibling of `python-edit-error-carries-no-handle-for-an-unnamed-variable`: both are the Python half of FORK-7's doors.

