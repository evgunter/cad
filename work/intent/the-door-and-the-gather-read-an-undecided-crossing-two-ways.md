---
id: the-door-and-the-gather-read-an-undecided-crossing-two-ways
kind: issue
title: The census door counts an undecided crossing as interference; the gather's interference read does not
status: closed
opened: 2026-10-16
priority: P2
cost: E
closed: 2026-10-16
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

## 2026-10-16 — closed (INTENT, `intent/j-census-reading-and-finite`)

**Which reading is right: both, for different questions.**
- The door asks whether the census cleared a pair. It must not record
  a touch between two solids the census did not clear, because it reads
  their overlap off those touches. So an undecided crossing and a
  `CensusUndecidable` pair belong in its set.
- The gather asks whether the census *decided* interference. Only then
  does it localize an overlap, and only that overlap can an assertion
  quiet. An undecided verdict must stay a refusal, unless it rides as
  evidence on a pair the census did decide interfering, which
  `partition` already does.

Widening the gather to the door's set would localize, and let an
assertion quiet, a pair the census never saw. So the gather's narrower
set is right, and so is the door's wider one.

**One home.** Both readings now live in topo:
- `census_interference_solids`: the decided reading;
- `census_uncleared_solids`: the decided reading, plus an `Undecided`
  crossing and `CensusUndecidable`;
- `census_cell_solid` and `census_entity_solid`: one cell→solid map.

`record_at_rest` calls `census_uncleared_solids`, which is exactly its
old `interpenetrating`. The gather calls the other three and drops its
`interference_solids`, `cell_solid` and `solid_of`.

**One behaviour change.** The gather's `solid_of` read only a solid or
a face. A `CensusUndecidable` naming an edge or a vertex now finds its
pair, so it rides as evidence on a pair the census decided interfering,
as one naming a face already did.

A topo row pins both readings, finding by finding. No golden moved; the
perf12 drift on main is FUSE's `pass-five-prime` item, unchanged here.
