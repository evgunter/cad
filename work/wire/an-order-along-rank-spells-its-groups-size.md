---
id: an-order-along-rank-spells-its-groups-size
kind: issue
title: Qualifier::OrderAlong { rank, of } spells the group's size in the name, so a third same-sense crossing renames the first two
status: open
opened: 2026-10-20
priority: P2
cost: M
refs: [naming-has-no-single-strategy-and-each-fork-patches-one-entity]
---


Found independently by both FORK-NAMESTRATEGY designers (2026-10-20).
- `crates/editor-core/src/names/role.rs`, `Qualifier::OrderAlong`. Its doc
  says `of` is "How many crossings the ordering ranked". The producer is
  at `crates/editor-core/src/names/emit_topo.rs` ~4946; there are `of: 2`
  literals at `emit_union.rs` ~3599 and ~3763.
- A sibling count in a name breaks locality: N2 and N5 say a piece's name
  "holds no count". A third same-sense crossing renames the first two even
  when it lands after them along the line.
- Repair: drop `of` and keep `rank`. Row 73 ruled on the rank; the count
  was never ruled on separately. N5's group-size rung reads the recorded
  group, not the name.
