---
id: memo-per-lane-input-lists-are-held-by-prose
kind: issue
title: memo.rs's per-lane input lists are held by prose, and nothing checks them against what each lane reads
status: open
opened: 2026-11-21
priority: P3
cost: M
refs: [the-patch-memo-key-misses-the-faces-across-a-trimmed-face, 7241]
---


Filed by the memo-holders unit (PR 7241) at its review's request.

## What

`crates/mesh/src/memo.rs`'s module docs (*The key, per lane*) list
the inputs each lane reads, and `FaceInputs::key` folds that list. The
list is held by prose alone. Nothing mechanical checks it against what
each lane's code reads off the body, the chord pass or the route.
`memo::tests`' ROWS table and `tests/patch_memo.rs` pin each LISTED
input. An input missing from the list has no row, so it goes
unnoticed.

One PR found two instances:

- **The face across each edge.** The trimmed lane's shared-side rule
  read it (`trimmed::faces_across`), and the key did not. Folded in
  PR 7241 (`memo::EdgeInputs::across`).
- **The face's sense.** The trimmed lane read it
  (`trimmed::trim_orientation`, the analytic row's `outward`), and
  the key folded it on the curved lane only. Folded in PR 7241. Before
  that, a memo warmed on a body and replayed on the same body with
  one trimmed wall's sense flipped answered `Ok` where a fresh run
  refused `TrimLoopOrientation` (`patch_memo.rs`,
  `the_trimmed_lane_misses_when_only_the_sense_flips`).

## Owed

A structural tie between what a lane reads and what the key folds,
so that an input cannot be read without being listed. One option: the
lanes take their face's inputs through `FaceInputs` (or a view of it)
rather than through `&Body` and `&ChordPass`. Another: a gate over the
lanes' reads.

## 2026-11-23 — one list entry retired (TESSLANE, branch `tesslane/join-split`)

The trimmed lane no longer reads the face across an edge, and `memo::EdgeInputs::across` and its fold in the key are gone with the rule that read it; the list in `memo`'s docs says so. `patch_memo::the_trimmed_lane_hits_when_only_the_face_across_an_edge_changes` holds the hit.
