---
id: the-edit-and-persist-doors-take-a-tolerance-no-check-reads
kind: issue
title: apply, apply_replayed, Doc::apply, save, load and canonical_bytes take a tolerance no check reads since the doors read no value
status: open
opened: 2026-11-18
---


Found by the value-free-door build (#7075's `[recipe.doors]`). Once the
edit door's profile replay, its loft station and degree checks and the
load door's program probe moved to evaluation, no check in either door
reads a tolerance. The internal door functions lost the parameter
(`check_written_node`, `write_edit`, `apply_spaced`, ... in
`crates/editor-core/src/edit.rs`; `validate_document` in
`crates/editor-core/src/persist/check.rs`). The public entry points kept
it, now unread (`_tol`): `edit::apply`, `edit::apply_replayed`,
`Doc::apply`, `persist::save`, `persist::load`, `persist::canonical_bytes`.
Dropping it is a signature change across several hundred call sites in
every crate, the demos, the Python binding and the guide, too wide to
ride the build.

`DocSession` still holds a `tol` for the solve and evaluation it runs
(`crates/editor-core/src/edit.rs`, `apply_spaced` callers), which is
not the door's.

**Done when** the six entry points take no tolerance, or one of them
reads it again for a reason a doc comment states.
