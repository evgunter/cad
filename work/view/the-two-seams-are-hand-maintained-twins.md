---
id: the-two-seams-are-hand-maintained-twins
kind: issue
title: evalseam is two modules with one coalescing machine copied four times, and nothing in it says so
status: review
opened: 2026-09-05
branch: view/seam-twins
pr: 2666
priority: P1
cost: D
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


## The count, re-derived — the item was stale in its central number

Read on `origin/main` at `d71bb6a785`. The fit seam landed after this
row was written (#2606), so the file holds **three** seams, not two:
three traits (`EvalService`, `IndexService`, `FitService`), three
`Inline*` impls, three `Thread*` impls, **three `dispatch` bodies** and
**six `busy` bodies** (plus the three trait declarations). The
coalescing machine is written **six** times, not four, and the module
header's *"Both implementations do this, by the same mechanism"* was
wrong twice over — about the number and about there being one
mechanism.

The test half was stale the same way: the row says *both* threaded
rows carry the 10 000 × 1 ms spin harness. `crates/viewer/tests/
eval_seam.rs` carries **six** copies of it, in three shapes, and four
more sites live in three other test files.

## The fork, decided: the generic worker, not the split

The two candidates are not the same fix and only one of them touches
what can silently diverge.

**Taken: a generic worker over a job trait.** The threaded half is now
one `Coalescing<J>` handle — at-most-one-outstanding, latest-wins, the
worker-gone reset and `busy() == running || waiting.is_some()` written
once — plus a private `Job` trait carrying the ONE rule that differs
between the seams, `supersedes`. Each threaded seam keeps only what is
genuinely its own: the evaluator's cancel token and its bounded join,
the index and fit seams' key comparisons.

**Not taken: splitting the file.** It removes no copies. The header's
disjointness is a symptom of the seams being three restatements of one
shape; naming the shape removes the restatements AND most of the
disjointness, where a split would have left three files each carrying
its own copy of the machine and a fourth seam free to add a fourth.
`crates/viewer/README.md`'s *The seam modules are a chain* also makes
one-file-owns-every-thread the property that won the current module
shape, so a split is a design change and not a tidy-up.

**The inline half is not a copy and is left alone.** An inline seam's
whole machine is one `Option`: `Option::replace` IS latest-wins, `busy`
is the slot, there is no worker to be ahead of and no `running` flag to
disagree with anything. Nothing in it can diverge without changing the
field's type. The prose that said it three times is now said once, in
the module header.

## Where the invariant lives now

`Coalescing`'s doc comment, in `crates/viewer/src/evalseam.rs`'s
threaded module — all four clauses, in one place, beside the two fields
they are about. A fourth seam diverging from it is no longer expressible
without writing a second handle: it would have to re-declare
`to_worker`, `from_worker`, `running` and `waiting` and re-implement
four methods, rather than get one of them subtly wrong.

## Residue

- The four wait-loop copies outside `eval_seam.rs`:
  `work/view/threaded-seam-wait-loops-are-hand-copied-across-four-test-files`.
- The coalescing rule is still prose no arbitrary implementor is held
  to — `work/view/evalservice-coalescing-rule-is-prose-no-implementor-is-held-to`
  is unchanged by this, and is now the only place the rule is
  unenforced.
