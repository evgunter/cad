---
id: a-split-piece-whose-lineage-runs-through-a-killed-piece-is-read-as-a-derived-chord
kind: issue
title: A split piece whose lineage runs through a killed piece reaches the derived-chord path
status: closed
opened: 2026-10-08
priority: P3
cost: M
closed: 2026-10-12
pr: 4794
branch: emit/curved-rim-chord
---


## What

Found while root-causing
`a-chord-across-a-curved-rim-cut-in-pieces-refuses-shared-rim`.
`emit_topo::name_boolean_edges` chases each result edge's `SplitEdge`
lineage to an operand-table key (`chase_edge_to_table`, which is
`topo::Body::split_root`). A root no table names is taken to be "a
join-minted crossing chord ... a DERIVED seam edge" (the comment above
the `resolves` test, near `emit_topo.rs:1390`). Then `chord_kind`
renames the edge from its two faces' shared rim.

In that row's document (the arc prism notched at 200°, united with the
post x ∈ [0.6, 0.8]), the arc piece from 53° to 197° is no chord. It is
a split piece of the operand rim `8v1`, from 0° to 197°. Its lineage runs
`26v3 → 11v5 → 8v1`, where `11v5` is the 37°–53° stretch inside the
post, which the boolean killed. The kill drops the edge's birth record
(`euler_kill.rs`'s `edge_provenance.remove`), so the chase stops at
`11v5` with no record. The same post across the second piece splits
`10v1` so the killed stretch is not on the survivor's chain, and the
lineage resolves. The boolean records killed chains (`discard.rs`'s
`boundary_chains`) and `names::borders` falls back to them
(`dead_split`), but this chase does not.

Since that row's fix, `rim_holding` gives the same piece both ways, so
nothing refuses. But a piece's name now depends on which path read it.
The comment states an invariant that does not hold, and any face pair
`rim_holding` cannot settle (several pieces with no carrier reading)
would refuse where the lineage would name.
`the-split-edge-lineage-walk-has-four-homes` (origin) is the walk
consolidation. This row is the missing fallback that walk would need.

## What a fix looks like

Give the boolean edge pass the killed chains (as `borders` has them),
or keep killed edges' `SplitEdge` records reachable, so the chase
reaches `8v1`. The row's first-piece document is the witness: with the
fix, `26v3` resolves in the operand table, and `rim_holding` is not
consulted.

## Fixed on `emit/curved-rim-chord` (PR 4794)

`emit_topo::KilledSplits` reads the discard rows' `boundary_chains`
into the hops the kills dropped: rows of an operand whose clone is the
arena are in arena keys, and a grafted B's rows are in B's keys.
`chase_edge_to_table` steps through them where the lineage stops at an
unnamed key. `chase_b` carries the walk on in B's keys from the
operand key it reads back. The comment above the `resolves` test now
says what holds: a root no table names was not split off an operand
edge.

Evidence. In each pair, the first route is with the fallback switched
off and the second is with it on:
- In the arc row's first-piece document, `26v3` (members in order) and
  `40v1` (flipped) went `11v5`, unnamed, to `chord_kind`. Both now
  resolve to `8v1`.
- In the circle rows, `23v3` went `8v5`, unnamed, four times. It now
  resolves to `5v1`.
- In the cavity row (`a_cavity_cut_by_the_notched_prism_names_its_rim_pieces`),
  the grafted-B lane's `43v3` went `12v9`, unnamed. It now resolves to
  B's `8v1`.

Over the whole editor-core suite, instrumented, 8084 edge reads
resolve through a killed stretch where they did not before. Each lies
within the operand edge it now names (`chord_within`), and each of
those edges is named in its table. No read failed either check.

After the fix, `rim_holding` is reached by one integration row,
`emit_union_borders::a_curved_divider_names_as_the_pair_boolean_does`.
That row's chord is a straight join-minted chord. No integration row
reaches it with a curved chord, so the unit rows witness its curved
reading.

The unit witness is `a_lineage_through_a_killed_piece_reaches_the_table`.
It splits a cube lateral twice and kills the middle piece. Without the
rows the walk stops at the killed key; with them it reaches the named
lateral. It is red when `KilledSplits::of` reads no rows.
