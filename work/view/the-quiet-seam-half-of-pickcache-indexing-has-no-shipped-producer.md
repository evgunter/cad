---
id: the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer
kind: issue
title: PickCache::indexing's seam half, and Coalescing's orderly-forget arm, have no shipped producer now that a crash panics
status: closed
opened: 2026-09-17
priority: P3
cost: D
closed: 2026-09-21
branch: view/seam-residue
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

## Closed: the seam half is deleted, the forget arm stays

Taken with its sibling, as both rows asked.

**`PickCache::indexing` is the cache's own record alone.** The
`&& self.seam.busy()` conjunct is gone, and with it
`NotIndexed::Absent`'s third cause and the both-halves prose.

The unreachability argument, which is an invariant rather than a
grep: an attempt is recorded `Asked(k)` in the same step of
`PickCache::sync` that submits `k` to the seam, and from there the seam
holds that request — `running` or `waiting` for `ThreadIndexer`,
`pending` for `InlineIndexer` — until it hands back the answer, which
`PickCache::pump` takes straight to `PickCache::land` in the same
expression, and `land` for a matching key moves the attempt to
`Answered`. So `Asked(k)` implies `busy()` for both shipped
implementations. The three ways the seam can drop a job without
answering are `Coalescing::poll`'s non-superseding arm (where the
answer in hand carries the same key and is landed in the same
statement), `Coalescing::close` and `Coalescing::forget_worker` — and
the last two are reachable only after `close`, which three `fn drop`s
are the only callers of. The one implementation that could be idle
under a standing `Asked` was `tests/frame_policy.rs`'s `DyingIndexer`,
deleted with the sibling row.

**`IndexService::busy` stays in the trait**, and the row's first
question was posed on an incomplete census: the conjunct was not its
only reader. `tests/eval_seam.rs` reads it directly
(`the_index_seam_answers_with_the_key_it_was_asked_with`) and through
the `Drainable::working` adapter that every threaded row in that file
spins on. It is the seam's own observable for *at most one
outstanding*, which is a contract about the seam and not about a
consumer.

**`Coalescing::forget_worker` stays too, and so do the two arms that
reach it.** Its unreachability on a running application is real and
already stated at the function — `close` is called from three `fn drop`
bodies and nowhere else — but answering a closed channel with
`Coalescing::crashed` would be a false statement about an orderly
shutdown, and would put back the conflation #2762 removed. That is the
same reasoning the module already applies one arm over, at
`a_send_that_fails_with_the_channel_still_ours_is_a_crash_too`: an arm
stays because the condition means what it means. The row
`a_closed_channel_is_forgotten_rather_than_announced` executes it;
pointing `dispatch`'s `None` arm at `crashed` reds exactly that row.

What proves the deletion safe: 714 rows green with the conjunct gone,
and the surviving half of the argument mutated in both directions —
`indexing()` forced `false` reds
`an_answer_for_a_superseded_generation_is_discarded_not_installed` and
`between_a_submit_and_its_answer_there_is_no_index_to_pick_from`;
widened to `self.attempt.is_some()` it reds
`a_refused_index_is_attempted_once_per_generation_and_not_once_per_frame`,
`between_a_submit_and_its_answer_there_is_no_index_to_pick_from` and
`replacing_the_document_drops_a_current_index_with_no_build_in_flight`;
and dropping `*attempt = None` from `PickCache::forget` — the orphan
direction, which is why the record and not the seam is the authority —
reds `a_build_in_flight_when_the_document_is_replaced_installs_nothing`
and `an_unsettled_delta_submits_nothing_and_drops_the_index_it_held`.
