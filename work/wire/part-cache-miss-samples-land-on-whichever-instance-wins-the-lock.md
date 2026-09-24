---
id: part-cache-miss-samples-land-on-whichever-instance-wins-the-lock
kind: issue
title: under the parallel node map, a part-cache miss path's probe samples splice at whichever instance won the lock
status: open
opened: 2026-09-24
priority: P4
cost: D
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
its lock across the miss path, so when two instances of one part sit in
the same level, the part is evaluated once, on whichever worker took the
lock first. Its verdicts are shielded (`_shield`) and land on no node,
but its `probe` samples go to the sink installed on that worker, which
is the winning instance's detached sink, and they are spliced at that
instance's slot. The serial walk gives them to the first instance in
`sched.order`. So on a document with two instances of one part in one
level, the MULTISET of samples is the serial walk's at every width, and
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
population" for documents that instantiate one part twice in a level.
The first keeps the stronger claim, and it has to decide what a
nested evaluation's own fold owes the outer one.
