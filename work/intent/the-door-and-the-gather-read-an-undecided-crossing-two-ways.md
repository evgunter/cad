---
id: the-door-and-the-gather-read-an-undecided-crossing-two-ways
kind: issue
title: The census door counts an undecided crossing as interference; the gather's interference read does not
status: open
opened: 2026-10-16
priority: P2
cost: E
---

Two readings of "this at-rest finding is an overlap between two solids"
disagree:

- the census door's `interpenetrating` (`crates/topo/src/census.rs`) maps
  `InstanceInterference`, `CensusUndecidable { a, b }`, an `EdgeFacePierce`,
  and an `EdgeEdgeCross` whose side is `SameSide | Undecided`;
- the gather's `interference_solids` (`crates/editor-core/src/checks/at_rest.rs`)
  maps `InstanceInterference`, an `EdgeFacePierce`, and an `EdgeEdgeCross`
  whose side is `SameSide` only, and not `CensusUndecidable`.

So the door withholds records from a pair it judged possibly interpenetrating
(an undecided crossing, or an undecidable pair) while the gather does not
report that pair as interference. Each should be one reading with one home,
in topo, which the gather calls. The gather also re-derives `cell_solid`,
which topo's census already has.

Found by the census-over-solids designers (PR 4666, 2026-10-16); the two
match arms are checked against main 4760a784.
