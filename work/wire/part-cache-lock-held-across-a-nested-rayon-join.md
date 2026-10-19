---
id: part-cache-lock-held-across-a-nested-rayon-join
kind: issue
title: PartCache::get holds a non-reentrant Mutex across a nested evaluation that joins on rayon; a stolen sibling instance can re-lock it on the same thread
status: closed
opened: 2026-09-25
priority: P1
cost: M
pr: 5273
branch: wire/parts-enter-before-the-map
closed: 2026-10-18
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

## Decided (2026-10-18, FORK-PARTCACHE, designers converged)

One Opus and one Fable designer, briefed with the problem only, reached
the same final state independently in round 1. The two rows are one
defect: evaluating a referenced part is work that belongs to no node, and
the top-level cache hands it to whichever node triggers it first, under a
lock (the serial walk's first instance, the parallel map's lock winner,
or — found by one designer — a placement, when the mate solve's reach
misses inside its recording unit). Below the top the code already states
the right rule: a document's parts are entered before it evaluates.

**The build:** the top evaluation enters its census
(`parts::instantiated`) in one serial pre-pass before the mate solve and
the node map, exactly as a nested evaluation does: each distinct
`(reference, ε)` evaluated at most once, in census (document) order, under
the existing shielding bracket, its probe samples reaching the sink ahead
of every node's. Successful rows are carried on the `Evaluation` and reused
from the prior (the node memo's A4/D9 argument; faulted rows are not
carried), so an unchanged assembly evaluates no part and a pose edit no
longer re-evaluates its part. `PartCache` splits into a serial `&mut`
store and an immutable, lock-free `Reached` table the node map and the
solve read by `&`; a miss refuses `NotEntered` at every depth. The mutex,
the top's evaluate-on-miss branch and the poisoned-lock arm go, as does
`EvalOptions::parallel`'s exception paragraph. The edit door's
`PartReach` stays a lazy store outside any evaluation, made `!Sync`. The
pre-pass hands the caller's cancel token through (today
`evaluate_entered` builds a fresh one).

Struck from this row's own sketches: a fresh single-thread pool does not
prevent the steal (rayon-core 1.13 `Registry::in_worker_cross` keeps the
caller's worker stealing from its own pool while it waits). Two premise
corrections: two instances in one level suffice, and instances are not
all in level 0 (`Doc::upstream_of` makes a mated instance depend on its
mates' reads). No ratified text is touched (the parts.rs module doc and
the `parallel` paragraph are agent-written). Not Ev's.

Signed (WIRE orchestrator).

## Closed (2026-10-18)

PR 5273 (merged `7e636c2bca`) built FORK-PARTCACHE's decision: the top evaluation enters its parts in one serial pass before the mate solve and the node map, rows are carried across evaluations from the prior, and the cache is an immutable lock-free table (`Reached`); the mutex and the lazy miss path are gone. A nested three-level fixture pins the serial walk's sample sequence under the parallel map (red on the old code at 3 threads); `PartReach` is `!Sync` by a `compile_fail` doctest. Full review approved with fixes; fix pass taken. CI fully green. Residue: `solve-runs-over-a-partly-entered-part-table-after-a-cancel`.

Signed (WIRE orchestrator).
