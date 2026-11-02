---
id: a-picture-with-no-build-reports-the-previous-pictures-memo-counts
kind: issue
title: a picture whose every copy refuses before a build reports the previous picture's memo hit and miss counts
status: open
opened: 2026-11-01
priority: P4
cost: C
---


## Finding

Found by CHROME's partial-picture lane (`work/chrome/a-failed-feature-hides-every-later-edit.md`), in `crates/viewer/tests/index_memo.rs`.

`crates/editor-core/src/resolve/pick.rs`, `PickMemo`: the per-picture counts (`node_hits`, `node_misses`, `table_hits`, `table_misses`) are reset by `PickMemo::open`, which `NodePick::build_with` calls only after `eval.usable(node)` and the body lookup succeed. A picture in which no copy gets that far (every drawn copy failed or poisoned) never opens, so after its `end_picture` the counts still describe the PREVIOUS picture, while the node map has been evicted to this one's (empty) size. The docs (`PickMemo::end_picture`: "the counts then describe the picture just closed") say otherwise.

Seen as `axis_projection after the second edit`: zero parts, `node_hits + node_misses == 1`. The viewer's `evalseam::MemoReport` carries these counts on every `IndexDone`. The viewer's index rows now skip their exact-count assertions on any picture holding an undrawn copy (`index_memo.rs`, `whole`), so nothing reds on it today.

Likely fix: call `PickMemo::open` at the top of `build_with`, before the refusals, or have `end_picture` zero the counts of a picture that never opened.
