---
id: doc-module-header-promises-a-door-it-does-not-hold
kind: issue
title: pncad-py — py/doc.rs's module header names evaluate (which lives in py/value.rs) and 3 of the 12 classes its register adds
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

Found in the S415 style review (2026-09-15).

`crates/pncad-py/src/py/doc.rs`'s module header opens:

> The document surface: `Doc`, `DocEdit`, `Node`, `evaluate`.

and a line later says Python "speaks Doc/DocEdit/evaluate/persist".
Both sentences are read as an inventory of what the module holds, and
both are wrong in the same two ways:

1. **`evaluate` is not in this module.** It is
   `crates/pncad-py/src/py/value.rs`'s `pub(crate) fn evaluate`. A
   reader following the header to find it does not.
2. **The inventory is 3 of 13.** `register` adds twelve classes —
   `NodeId`, `Doc`, `DocEdit`, `ParamName`, `DocParam`,
   `DocParamValue`, `Node`, `SketchPlane`, `BooleanOp`, `PartSelect`,
   `TubeWindow`, `Loaded` — plus the `load` function. The header names
   three of them.

This is the failure mode a hand-maintained count has: the header was
right when it was written and every class added since arrived without
it. The repair is either to stop enumerating (name the module's
subject, not its contents) or to enumerate from `register`, which is
the list that is actually true by construction.

## Where to look

- `crates/pncad-py/src/py/doc.rs` — the `//!` header, and `register`
  at the foot of the file.
- `crates/pncad-py/src/py/value.rs` — `evaluate`, where it lives.
