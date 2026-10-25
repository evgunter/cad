---
id: a-flush-row-is-quieted-by-no-assertion
kind: issue
title: A flush row is read by no quieting rule: the Gap measure that states it exists, and nothing reads it for the row
status: open
opened: 2026-10-23
---


The census writes a `SameOriented` flush row for two planar faces of
different groups on one plane, facing the same way, whose boundaries meet
while their interiors do not (`crates/topo/src/census.rs`
`flush_at_rest`, `[assertions]`). DESIGN says such a row is a finding
unless it is established, by a flag mate or by the replay.

The measure that states it exists. `Gap` between two parallel planes is
the signed offset along the outer face's outward normal
(`crates/editor-core/src/eval/measure.rs` `gap`), and `= 0` reads the same
in either role order. Nothing reads it for the row, though:
`ChecksConfig::unproven_coincidence` reads no assertion, so a flush row
cannot be quieted. The tour bench's two flush post ends are reported
for this reason.

Owed: the `unproven-coincidence` check quiets a flush row under an
assertion `Gap(a, b) = 0` naming its two faces, by stage 5's quieting
rule (`interference-at-rest-is-a-finding` owns that rule). Rows: a
flush row quieted by its assertion, and one left loud by an assertion
naming another pair.

Filed from PR 5605's dual review (R2).
