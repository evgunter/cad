---
id: a-booleans-same-step-split-and-join-loses-the-childs-lineage
kind: issue
title: A boolean step that splits an edge and joins the child away in the same step records no lineage for the child, so the Borders chase stops at it
status: closed
closed: 2026-10-14
opened: 2026-10-13
priority: P3
cost: M
---


## What

`Obstacles::record` (`names/borders.rs`) keeps `split_from` lineage in two ways:
- from each step's edges that are live at the end of that step;
- from the discarded faces' boundary chains.

Suppose a boolean step splits an edge, and its own output stage joins the new child away as `gone`. Neither source holds that child. A chase that goes from `kept` to `joined_from` then stops dead at it.

The Split op exports exactly this lineage (`SplitNaming::joined_lineage`, topo `splitting/finish.rs`), but `BooleanNaming` has no equivalent.

This was found by the PR 4949 review. The corpus does not reach it, so it is open whether any document does.

**Owed:**
- a fixture;
- a joined lineage export from the boolean, mirroring the Split's.

## Closed

**The record.** The boolean's output stage copies each joined-away
edge's `SplitEdge` record before the join kills it:
`topo::BooleanNaming::joined_lineage` (filled in `edge_join::join_stage`,
carried by all three `finish_output` callers), the Split's
`SplitNaming::joined_lineage` mirrored. Record only: no geometry or
join decision changes. `Obstacles::record` (`names/borders.rs`) adds it
to `split_from`, so the chase from `kept` through `joined_from`
continues up the child's lineage.

**Fixtures.**
- `topo` `boolean_discards::an_edge_split_and_joined_away_in_one_step_records_its_parent`:
  two bricks overlapping along x union into one, and the join kills
  pieces the cut minted. Red with the copy disabled (the map comes back
  empty).
- `editor-core`
  `emit_union_borders::an_earlier_seam_split_and_joined_away_in_one_step_names_alike`:
  the right piece's edge along the walls reaches the discards {1, 6}
  with the record and {6} without it, measured by a probe that
  compared the chase with and without the copied hops.

**What moved: nothing.**
- The editor-core `ci` suite records the case 8566 times (a probe
  counted `joined_lineage` entries absent from `split_from`), the corpus
  included.
- A second probe compared each piece edge's reach with and without the
  copied hops across the whole suite. It fired on no row.
- In the fixture above the reach grows, but both discards are already
  one obstacle, so no root and no name moves.

That is expected. A discard the copied hop adds borders the joined edge
on the same side as the discard already reached. They are one region
unless the parent's own boundary passes through the joined-away vertex.
That exception is
`a-borders-join-can-link-two-obstacles-that-meet-only-at-a-notch-apex.md`'s
question, not this row's.
