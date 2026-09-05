---
id: indexing-seam-outlives-a-worker-panic
kind: issue
title: A panicked seam worker leaves the chrome promising an answer that is not coming
status: open
opened: 2026-09-05
---


## What

Found by VIEW-6b's correctness review, and **inherited rather than
introduced**: the evaluation seam has the same shape and had it first.

`ThreadIndexer` and `ThreadEvaluator` both notice a worker that has
gone — a `TryRecvError::Disconnected` in `poll` clears `running` and
`waiting`, so `busy()` correctly answers `false` afterwards. What
neither consumer does is READ that.

- `PickCache::indexing` (`crates/viewer/src/pick.rs`) reports its own
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
