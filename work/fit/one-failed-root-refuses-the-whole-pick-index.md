---
id: one-failed-root-refuses-the-whole-pick-index
kind: issue
title: one failed root refuses the whole pick index, so every healthy body loses picking and the picture stops following the document
status: open
opened: 2026-09-25
priority: P2
cost: D
refs: [a-derived-pick-index-failure-outshouts-its-cause]
---


Found by VNEWS's lane on `a-derived-pick-index-failure-outshouts-its-cause`
(Ev's report from the dumbbell document), while tracing what the
pick-index badge's refusal actually costs a reader.

## Finding

`crates/viewer/src/pickindex.rs`, `PickIndex::assemble`: the walk over
`doc.roots()` skips a root whose build answers
`NodePickError::NotABody`, and returns `PickIndexError::Node` for **any
other** refusal — including `NodePickError::Standing`, which is what a
root with no `Ok` value (failed, poisoned, never ran) answers
(`editor_core::resolve::pick`'s `standing_value`). So one failed root
refuses the whole index, not just its own bodies.

What that costs, in `crates/viewer/src/app.rs`'s `ViewerApp::sync_scene`:

- **Picking stops on every body**, the healthy roots' included: with no
  index, every `Select` is refused (`pickcache::unindexed`,
  `NotIndexed::Absent`).
- **The picture stops following the document.** The scene is drawn
  from the index (`index.scene_focused`), and `sync_scene` returns on
  `CacheStep::Held` before the rebuild, so the viewport keeps whatever
  mesh it last built — for an Open whose document already has a failed
  root, nothing.

Reproduced headlessly over Ev's `dumbbell.pncad` edit log (replayed
through `DocSession` and `PickCache::inline`): after each Boolean the
kernel refused (nodes 11, 12, 13, 18, 19), every other root (3 and 7,
and 10 where it was still a root) was `Ok`, and the index refused on
the failed one.

## The question for FIT

A root with no value has nothing to draw and nothing to pick, which is
the same reason `NotABody` is skipped. Should a `Standing` refusal be
skipped too, so the index — and the picture drawn from it — covers the
roots that did evaluate? The failed root is already reported, loudly,
by the feature tree (`tree::RowStatus::Failed`), and since
`frame::index_badge` now draws this refusal as a consequence of that
row (the VNEWS fix), skipping it would retire the badge for this case
entirely rather than only quieting it.

The case against, for whoever takes it: a picture of the healthy roots
alone is a picture of a document that does not exist (the product the
recipe denotes does not gather), and the scene currently shows either
the whole product or the last one that was whole. That is a decision
about what the viewport promises, not a bug fix, so it may want Ev.
