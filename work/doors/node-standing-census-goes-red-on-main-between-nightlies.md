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

## 2026-11-11: a third time (PCERT's nightly-reds lane)

Nightly 42965378060 (main `b2916558d9`) is red on `every_node_result_reader_goes_through_usable_or_is_listed`, at default ε and 1e-6. The tree holds `crates/editor-core/src/eval/mod.rs` at 35 reads against `READERS`' 33, and `crates/editor-core/src/eval/parts.rs` at 2 against 1. The move is PR 6735 (`onecopy/dual-tier3`, merge `1ed46561`): the two files' `NodeResult` mentions go 34 → 36 and 1 → 2 between `1ed46561^1` and `1ed46561`, and no later merge in the window touches `parts.rs`. Still red on main `019bed88`. The class is now on green's `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.
