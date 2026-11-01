---
id: one-failed-root-refuses-the-whole-pick-index
kind: issue
title: one failed root refuses the whole pick index, so every healthy body loses picking and the picture stops following the document
status: closed
opened: 2026-09-25
priority: P0
cost: M
refs: [a-derived-pick-index-failure-outshouts-its-cause]
parent: a-failed-feature-hides-every-later-edit
branch: chrome/partial-picture
rides_with: a-failed-feature-hides-every-later-edit
pr: 6205
closed: 2026-11-01
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

## Parked on the D10 hold (2026-10-22, FIT orchestrator)

The walk this row is about has changed since it was filed and is about
to change again. `PickIndex::assemble` no longer walks `doc.roots()`:
it walks `doc.placements()` filtered to the world's space and to each
loose copy's space (`Space::World`, `Space::Loose`, `eval.loose_copies`),
and still returns `PickIndexError::Node` on the first placement whose
build refuses. Stage 3 D′ (`work/intent/a-placement-pins-by-written-values-without-a-mate.md`,
`[ev]` PR 5494) retires loose spaces and decides what the viewer shows:
a set of spaces, the world and the sinks by default, drawn through one
composing door shared with export. This row's question (does the
picture cover what evaluated when the product does not gather?) is a
question about that shown set, so it is asked again once D′ lands,
against the walk D′ leaves, and not before.

## Moved (2026-10-26)

FIT folded back into VGEOM (Ev, in chat, 2026-10-26): FIT was cut from
VGEOM on 2026-09-20 to run in parallel, a reason that is gone at FIT's
size. Id and body unchanged.

## Released (INTENT, 2026-10-26)

This row was parked on the stage-3 placement pivot (`a-placement-pins-by-written-values-without-a-mate`). All three of its PRs are merged: D′1 (PR 5592), D′2 (PR 5613) and D′3 (PR 5636, `29110056e8`). Re-check this row against main, then fix, narrow or close it. That is the owning track's call.

## Closed (2026-11-01, PR 6205)

Closed with its carrier, `a-failed-feature-hides-every-later-edit`: the index no longer refuses as a whole on one failed copy.
