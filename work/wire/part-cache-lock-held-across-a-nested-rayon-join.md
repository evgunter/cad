---
id: part-cache-lock-held-across-a-nested-rayon-join
kind: issue
title: PartCache::get holds a non-reentrant Mutex across a nested evaluation that joins on rayon; a stolen sibling instance can re-lock it on the same thread
status: open
opened: 2026-09-25
priority: P1
cost: D
---


## The finding

`PartCache::get` (`crates/editor-core/src/eval/parts.rs`, in `get`
beside `let mut entries = match self.entries.lock()`) holds a
`std::sync::Mutex` across the miss path on purpose, so that a part two
instances race for is evaluated once. Its doc argues the lock "is never
re-entered" because a nested evaluation builds its own cache. That
argument covers the NESTED evaluation. It does not cover the rayon
worker that holds the lock.

The miss path runs a whole evaluation of the part (`evaluate_nested`,
with `parallel: false`). That evaluation still reaches rayon maps
inside the kernel. Solid validation's check 7 runs
`topo::props::sign_walk`, which decides faces through
`geom_core::k_stats::map_detached`, an indexed `par_iter` whose
`collect` joins. A rayon worker blocked in a join does not sleep: it
runs other pending jobs, including jobs stolen from other workers'
deques. Under `EvalOptions::parallel` every `InstantiatePart` node sits
in level 0 (it has no inputs), so the outer level map has queued one
job per instance. If the worker holding the lock picks up a sibling
instance's job while it waits in the nested join, that job calls `get`
for the same part and locks the same non-reentrant mutex on the same
thread, which deadlocks (or panics; std leaves re-locking on the
holding thread unspecified).

Conditions: a document that instantiates one part at least three times
(the holder, one sibling blocked on the lock on another worker, and at
least one sibling still queued), a pool of at least three threads, and
a part whose evaluation reaches a rayon join while the lock is held.

**Latent.** `EvalOptions::default()` sets `parallel: false` and every
shipping caller takes the default. It **blocks turning `parallel` on**,
which is the switch `work/gather/`'s parallel-node-map unit made sound
for recordings and sessions.

**Not reproduced.** A throwaway row (not committed) evaluated a unit-cube
part instantiated 3, 6 and 12 times, `parallel: true` on a 4-thread pool,
20 rounds each, under a 60 s watchdog. It finished in 0.46 s with no
hang. So this is a derivation from rayon's documented work-stealing
semantics and the code, not an observed deadlock. Either the unit cube's
face walk is too cheap to leave a join pending long enough to steal, or
the schedule never put a sibling job where the holder steals from. A
repro likely needs a part with many curved faces, so that the nested
map outlasts the steal window.

## What a fix is

No thread may block on the part's lock or cell while it can be running
another job that needs that same part. A per-key `OnceLock` does not fix
this on its own: re-entering the initialisation on the same thread
deadlocks in the same way. What does fix it:

- **Evaluate the part outside the lock** and keep the first result
  inserted, dropping any racing duplicate. This trades "evaluated once"
  for no lock across the join, and it changes `part_evaluations`.
- **Run the nested evaluation on a fresh single-thread pool**, so no
  outer job can be stolen into it while the lock is held.
- **Resolve every distinct part before the level map starts.** The part
  set is known from the level-0 `InstantiatePart` nodes, so the cache
  can be filled serially, with no instance job in flight.

Which is right is a design question about what "once" promises. The
third keeps the promise and needs no lock across a join.
