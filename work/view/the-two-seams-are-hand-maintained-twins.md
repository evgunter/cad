---
id: the-two-seams-are-hand-maintained-twins
kind: issue
title: evalseam is two modules with one coalescing machine copied four times, and nothing in it says so
status: open
opened: 2026-09-05
---


## What

Found by VIEW-6b's style review (S1, S2, S21, S16 — one finding).

`crates/viewer/src/evalseam.rs` now holds two seams. What they SHARE
is the call into the expensive thing (`run_once`, `build_index`) and
`Generation`. What is **copied** is the part that carries the
invariant:

- `ThreadIndexer::dispatch` is a **verbatim** copy of
  `ThreadEvaluator::dispatch`, comment included, and neither site says
  so. The project's duplication sweep greps for `verbatim`,
  `ported from` and `mirror of` in prose, so this pair is invisible to
  it.
- The coalescing machine — at-most-one-outstanding, latest-wins, the
  worker-gone reset, `busy() == running || waiting.is_some()` — is
  written **four times** across `InlineEvaluator`, `ThreadEvaluator`,
  `InlineIndexer` and `ThreadIndexer`. The module header still says
  *"Both implementations do this, by the same mechanism"*; there are
  four.
- Both threaded test rows carry their own copy of the same
  10 000 × 1 ms spin harness (`crates/viewer/tests/eval_seam.rs`).

## And the file is two modules

Independently of the duplication: the file is two seams sharing
`Generation`, `SpawnError`/`Worker`, one `Send` assertion and a naming
convention. Three of its six header sections exist only to explain how
the second seam differs from the first — no cancel, its own worker,
a pair for a key. That is the shape
`work/view/frame-module-has-eight-concerns-and-no-holds-row.md`
records one unit later in its life: a module that accreted a titled
section per unit until the header was a table of contents for
unrelated things. Catching it one unit earlier is the only reason to
file it now rather than after the third seam.

## What a fix owes

Deciding which it is. A generic worker over a job trait would remove
the copies and cost a type parameter; splitting the file would remove
the header's disjointness and not the copies. They are not the same
fix and the item should not pretend they are — but the four
hand-maintained copies of one invariant are the part that can silently
diverge, and the two `dispatch` bodies are already identical enough
that a fix to one would not be applied to the other by anyone reading
either.
