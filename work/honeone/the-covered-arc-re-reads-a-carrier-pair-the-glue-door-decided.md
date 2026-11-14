---
id: the-covered-arc-re-reads-a-carrier-pair-the-glue-door-decided
kind: issue
title: topo: reduce::parents_distinct_from re-reads a cross-operand carrier pair over the extent the glue door already decided it on
status: open
opened: 2026-11-09
priority: P3
cost: E
---



Found by SEAL's `a-corner-reads-only-its-own-sectors` sweep (corner and vertex sites that read a carrier pair).

## What

`reduce::parents_distinct_from` (`crates/topo/src/boolean/reduce.rs:4067`, called at `reduce.rs:2301`) gates the covered-arc `LiesOn` lane on every parent face of the arc's edge reading `CarrierRelation::Distinct` against the other operand's face. It reads that through `carrier_pair::carrier_pair_relation` (`reduce.rs:4081`). That is `carrier_pair_decided` over `pair_extent` (`crates/topo/src/boolean/carrier_pair.rs:254`, `:278`, `:287`): the call the glue door makes for the same cross-operand pair (`crates/topo/src/boolean/glue.rs:84`).

So where the door read the pair, this is a second decision of one row on one extent. The door keeps no table of its `Apart` pairs (fork `glue-disagreements-carry`: no reader needed one), so the reader decides it again, and each decision is logged.

## Not checked

- Whether `x` and `y` at this site are always the bodies the door read, or operand clones whose faces the cuts have moved. If they are clones, the extent differs and this is a reading of its own.
- Whether the door reads every pair this site asks about (it reads candidate pairs only).

## Shape

Either the door keeps the verdicts this reader needs, or this reader asks a question of its own (the arc's parents against the face, over the arc's extent) and says why it is not the door's.
