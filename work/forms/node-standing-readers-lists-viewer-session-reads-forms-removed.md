---
id: node-standing-readers-lists-viewer-session-reads-forms-removed
kind: issue
title: node_standing gate red on main: READERS lists viewer session.rs at 3 reads that the forms change removed
status: open
opened: 2026-11-03
---


`crates/editor-core/tests/node_standing.rs` (`READERS`) lists
`crates/viewer/src/session.rs` with 3 readers of `Evaluation::result`.
FORMS' 3116c6afa6 / 6c1554fd01 ("the split and pattern forms take no
values") removed those reads, and `session.rs` on main has 0, so
`node_standing::every_node_result_reader_goes_through_usable_or_is_listed`
fails on main (the per-PR gate does not run it; the nightly will).
Found by WIRE's `wire-stop` lane, 2026-11-03.

Fix: drop the `session.rs` line from `READERS` and lower the array's
length by one.
