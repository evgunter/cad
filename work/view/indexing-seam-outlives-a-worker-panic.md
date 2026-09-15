---
id: indexing-seam-outlives-a-worker-panic
kind: issue
title: A panicked seam worker leaves the chrome promising an answer that is not coming
status: closed
opened: 2026-09-05
closed: 2026-09-15
branch: view/seam-panic
pr: 2637
---


## What

Found by VIEW-6b's correctness review, and **inherited rather than
introduced**: the evaluation seam has the same shape and had it first.

`ThreadIndexer` and `ThreadEvaluator` both notice a worker that has
gone — a `TryRecvError::Disconnected` in `poll` clears `running` and
`waiting`, so `busy()` correctly answers `false` afterwards. What
neither consumer does is READ that.

- `PickCache::indexing` (`crates/viewer/src/pickcache.rs`) reports its own
  `outstanding` field, which is cleared only by an answer that never
  arrives. So the toolbar spins on `indexing…` forever, requests a
  repaint every frame to collect a result nobody will send, and every
  click is refused `NotIndexed::Building` — *"the picture is still
  being indexed"*, which is false.
- `DocSession::busy` (`crates/viewer/src/session.rs`) is defined
  against the session's own two generations for a stated reason, and
  has the same consequence for a panicked evaluator: a permanent
  `evaluating…`.

## Why it is one item and not two

The fix is the same shape on both sides — a consumer whose "work is
outstanding" answer consults the seam it asked, so a seam that can no
longer answer stops the indicator instead of feeding it. Fixing one
and not the other would leave the viewer with two spellings of the
same state, which is what `crates/viewer/README.md`'s one-progress-
state rule exists to prevent.

## What it is NOT

Not a spinner-over-idle-work bug of the kind GUI-3 already ruled on:
there the seam is honest and the chrome reads it. Here the seam is
honest and the chrome does not ask.

## Cost

Unmeasured, and only reachable through a worker panic — which is a
bug in the build code, not an ordinary state. It is filed because the
sentence the user is shown is confidently false, not because it is
frequent.

## Closed

**The census was three seams, not two, and one of the two named was
already right.** `evalseam` grew a third seam (`FitService`, #2606)
after this was filed, so the population is evaluation, index and fit —
and of the three only the index consumer answered from its own
bookkeeping:

| seam | the consumer | consults the seam? |
|---|---|---|
| `EvalService` | `DocSession::running`, folded with `busy` into `Outstanding` | yes |
| `IndexService` | `PickCache::indexing` | **no — the defect** |
| `FitService` | `ViewerApp`'s two `FitService::busy` reads | yes |

The claim above about `DocSession::busy` was wrong when it was written,
not merely stale: no chrome reads `busy` alone. `frame::progress` has
taken the seam's answer beside it since before this item was filed
(`(true, false, _) => Canceled` at `bf4e3d16b5`, the head of the day
before), so a panicked evaluator has always reached the toolbar as
*canceled — showing an older result* rather than as a permanent
`evaluating…`. `busy`'s stated reason — it answers *am I showing the
current document*, which is true of a dead evaluator — is untouched by
the fix and did not need to be overridden. What answers *is anyone
working on it* is `running`, and that is the seam's own `busy`.

**The fix**: `PickCache::indexing` is
`outstanding.is_some() && seam.busy()`. Both halves, because either
alone is false in one direction — the record alone promises an answer
nobody will send, and the seam alone lights the indicator for a build
`forget` has already orphaned (`frame_policy.rs`'s
`a_build_in_flight_when_the_document_is_replaced_installs_nothing`
holds that half).

**Residue**, filed rather than disclosed:
`a-dead-seam-worker-reads-as-an-ordinary-idle-state` — what the chrome
should say INSTEAD of the withdrawn promise, on all three seams. That
is a new typed fact and a badge decision, and it wants Ev.
