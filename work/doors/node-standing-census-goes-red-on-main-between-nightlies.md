---
id: node-standing-census-goes-red-on-main-between-nightlies
kind: issue
title: node_standing's census is a slow-set row, so main breaks it between nightlies
status: open
opened: 2026-11-02
---


`crates/editor-core/tests/node_standing.rs`'s
`every_node_result_reader_goes_through_usable_or_is_listed` counts
node-result reads per file against a `READERS` table. It runs in the
slow set, so a PR's CI does not run it, and main went red on it twice
in one day:

- `cc28b3997f` added two reads to `eval/members.rs` (they were in its
  `#[cfg(test)]` module; PR 6264 makes the census count shipped code
  only).
- `8187c0daf9` added four reads to `viewer/src/pane/create.rs` with no
  `READERS` line (PR 6292 adds it).

Both were found only by unrelated lanes whose local full runs tripped
over them, and each fix rode an unrelated PR. The census is a
seconds-long source scan, not a slow test: move it into the per-PR
set (or into the gates job) so the PR that adds a read is the one that
answers for it. Check its runtime first; if it is slow, the scan is
what to fix.
