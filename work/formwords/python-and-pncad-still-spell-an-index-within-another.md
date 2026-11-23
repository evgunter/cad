---
id: python-and-pncad-still-spell-an-index-within-another
kind: issue
title: Python's VarDecl.index and pncad's index_decl still spell a binder's second index as within
status: open
opened: 2026-11-23
---

The kernel stores a binder's further index as `IndexSeat::After(prev)`
(`crates/editor-core/src/var.rs`, `VarDecl::Index { after }`). Python and
pncad still spell it the old way:

- Python's `VarDecl.index(count, within=…)` (`crates/pncad-py/src/py/doc.rs`
  `VarDecl::index`, and the `within` getter).
- pncad's `pattern::index_decl(name, count, within)`
  (`crates/pncad/src/pattern.rs`).

Each lowers `within` to `after`, so the word no longer matches the
model. Rename it to `after`, or retire it in favour of
`Doc.each((m, n), …)` and the presets
(`python-writes-repetition-through-doc-each-and-binder-presets`).
