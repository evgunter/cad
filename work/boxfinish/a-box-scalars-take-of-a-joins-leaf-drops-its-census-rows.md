---
id: a-box-scalars-take-of-a-joins-leaf-drops-its-census-rows
kind: issue
title: A box scalar's take of a join's leaf keeps the body and drops the census's rows and findings its minted records cite
status: open
opened: 2026-11-22
priority: P2
cost: E
---


Found in review of PR 7124 (`embed/records`, 2026-11-22). At a box
scalar a `⊔`'s leaf is a replay holding its arrivals
(`Replay::of_assembled`), and its census runs where a taker finishes
it: `AtRestPolicy::take` in `crates/topo/src/props.rs` calls
`finish_assembled` and keeps `censused.body` alone. The census's rows
and findings are dropped there, so the records it minted on that body
cite `Decided(k)` rows no value holds, and what it found between the
arrivals reaches no check. The `⊔`'s own value was built from
`leaf_assembled`, whose `CensusedLeaf` is empty at a box scalar
because no census had run (`crates/topo/src/finish.rs`), and a reader
cannot tell that from a census that found nothing.

This is not the hole PR 7124's review found at the taker (a leaf's
records reaching its taker's gate, pinned by
`the_gate_confirms_every_record::a_box_scalars_taker_reads_the_records_its_leaf_holds`):
the records arrive. What is lost is the census's report. It retires
with the box-scalar leaf switch
(`work/boxfinish/delete-the-box-scalar-sweep-leaf-switch.md`) or
needs the taker to hand the report back to the value that owns it.
