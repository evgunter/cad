---
id: part-cache-miss-samples-land-on-whichever-instance-wins-the-lock
kind: issue
title: under the parallel node map, a part-cache miss path's probe samples splice at whichever instance won the lock
status: closed
opened: 2026-09-24
priority: P4
cost: M
rides_with: part-cache-lock-held-across-a-nested-rayon-join
pr: 5273
branch: wire/parts-enter-before-the-map
closed: 2026-10-18
---


## The finding

`evaluate`'s parallel arm (`crates/editor-core/src/eval/mod.rs`, the
`geom_core::k_stats::map_detached` call in `evaluate_at_descent`) runs
each node of a level under a K-funnel frame and sample sink of its own
and splices the recordings back in `sched.order`, so the `probe` sample
population is the serial walk's, element for element, at any thread
count (`crates/editor-core/tests/parallel_node_map_probe.rs`).

One thing that composition cannot order is a decision that belongs to
no node. `PartCache::get` (`crates/editor-core/src/eval/parts.rs`) holds
its lock across the miss path, so when a document instantiates one part
more than once, the part is evaluated once, on whichever worker took the
lock first. (An `InstantiatePart` node has no inputs, so every instance
sits in level 0 and they all race.) Its verdicts are shielded (`_shield`) and land on no node,
but its `probe` samples go to the sink installed on that worker, which
is the winning instance's detached sink, and they are spliced at that
instance's slot. The serial walk gives them to the first instance in
`sched.order`. So on a document that instantiates one part more than
once, the MULTISET of samples is the serial walk's at every width, and
the SEQUENCE is not: the miss path's samples move to the later
instance's slot whenever it wins the race.

Derived from the code, not measured: no row reads the sample sequence
of an assembly under `parallel: true`. The margin consumers
(`tools/k-lint`, `docs/K-REPORT.md`) read distributions, which the race
does not change, so this is an ordering claim about a recording rather
than a population or decision defect.

## What a fix is

Either the miss path runs under `k_stats::detached` and the cache keeps
the recording beside the row, to be spliced by the fold at the first
instance in `sched.order` that reads the key; or the statement the
evaluator makes about its recordings is narrowed to "the serial walk's
population" for documents that instantiate one part more than once.
The first keeps the stronger claim, and it has to decide what a
nested evaluation's own fold owes the outer one.

## Moved to WIRETAIL (2026-10-17)

WIRE's priority-seam cut moved this row, keeping its id and body: WIRE keeps its P0–P2 rows and the E rows that ride with them.

Signed (WIRE orchestrator).

## Moved to WIRE and decided (2026-10-18)

One defect with `part-cache-lock-held-across-a-nested-rayon-join`; FORK-PARTCACHE decided both (see that row). Rides with it.

Signed (WIRE orchestrator).

## Closed (2026-10-18)

PR 5273 (merged `7e636c2bca`) built FORK-PARTCACHE's decision: the top evaluation enters its parts in one serial pass before the mate solve and the node map, rows are carried across evaluations from the prior, and the cache is an immutable lock-free table (`Reached`); the mutex and the lazy miss path are gone. A nested three-level fixture pins the serial walk's sample sequence under the parallel map (red on the old code at 3 threads); `PartReach` is `!Sync` by a `compile_fail` doctest. Full review approved with fixes; fix pass taken. CI fully green. Residue: `solve-runs-over-a-partly-entered-part-table-after-a-cancel`.

Signed (WIRE orchestrator).
