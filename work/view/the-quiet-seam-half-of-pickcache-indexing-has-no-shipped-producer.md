---
id: the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer
kind: issue
title: PickCache::indexing's seam half, and Coalescing's orderly-forget arm, have no shipped producer now that a crash panics
status: open
opened: 2026-09-17
priority: P3
cost: D
---



## What

Residue of `a-dead-seam-worker-reads-as-an-ordinary-idle-state`, created
by the ruling that closed it (Ev, in-chat, 2026-09-17: *"panic on crash
is good"*).

`PickCache::indexing` is two reads ANDed:

    matches!(self.attempt, Some(Attempt::Asked(_))) && self.seam.busy()

and its doc argues both halves, each false in one direction. The seam
half's stated job is a worker that has gone — *"a worker that has gone
clears its own flags ... while the attempt stays `Attempt::Asked`"*.

**That producer is gone.** A shipped `ThreadIndexer` whose worker
crashes now panics on the UI thread at the point of detection, so it
never returns from `busy()` in that state at all, and an `InlineIndexer`
has no worker to lose. The only implementation that can still make the
seam half do work is `tests/frame_policy.rs`'s `DyingIndexer`.

The same holds one layer down for `Coalescing::forget_worker`, which is
now the orderly-shutdown path alone: `close` is called from `Drop` and
from nowhere else, so on a running application nothing reaches it.

## Why it was not taken with the fix

Because it is a removal, and removals of a guard want the argument made
on their own. Three things would have to be decided together:

- whether `IndexService::busy` stays in the trait at all, or whether
  the cache's own record is now the whole of `indexing`;
- what `Coalescing::forget_worker` and `Coalescing::close`'s pairing
  should be once only `Drop` reaches them — the join/no-join split
  between `close_and_join` and `close` is argued on shutdown latency
  and is not affected, but the flag-clearing is;
- and what happens to the two consumer rows in `frame_policy.rs` that
  drive the quiet-seam state, which is the sibling row
  `the-dying-seam-fakes-mirror-a-machine-they-do-not-share`.

None of that is what Ev ruled on, and doing it inside the ruling's own
unit would have buried it.

## What it would cost to measure

Nothing to measure. The producer question is settled by reading
`Coalescing`'s two detection arms and `close`'s two call sites (both
`fn drop`); what is open is a design decision about how much of the
both-halves rule survives losing one of its reasons.
