---
id: the-witness-edits-need-a-facade-type
kind: issue
title: the two witness DocEdit arms need a facade type before Python can build them
status: open
opened: 2026-09-09
---



Found at LIB-EDITS, which built two of `B-DOC-EDITS`'s five arms and
stopped on these.

`DocEdit::ReWitness { node, witness }` and
`DocEdit::ReWitnessBulk { entries, certification }`
(`crates/editor-core/src/edit.rs:156` and `:169`) carry
`editor_core::witness::WitnessDatum` and `BranchCertification`.
Neither type appears anywhere under `crates/pncad/src/` — the façade's
curated list (`crates/pncad/src/document.rs:34-46` and the
`pub use` blocks after it) does not carry them — and `pncad-py`
depends on `pncad` and `quantity` alone
(`crates/pncad-py/Cargo.toml`). So a `DocEdit.re_witness` constructor
has no type to take its argument in: this is the appearance four's
sentence at a second pair of types, not an oversight in the bindings.

The census records it that way now: the two rows stay in
`MEMBERS_NOT_BOUND` under `B-DOC-EDITS`
(`crates/pncad-py/tests/test_binding_census.py`), and the family's
charter says a curated payload comes first.

## What closing it looks like

A façade decision BEFORE a binding unit: whether a witness datum is a
value a consumer of `pncad::document` may name at all. Both types are
opaque `{ schema: u32, bytes: Vec<u8> }` records the kernel never
reads, so carrying them is carrying two integers-and-bytes pairs —
the payload rule `ProgramRefusal` and `AttrKind` moved under, applied
to an edit's ARGUMENT rather than to a refusal's payload. If they are
curated, the Python doors are mechanical and their four refusal tags
(`witness_on_non_sketch`, `duplicate_witness_entry`,
`empty_witness_bulk`, and `unknown_node`) become provokable from
Python. If they are not, the two rows are `different-shape` like the
appearance four and `B-DOC-EDITS` closes with them.
