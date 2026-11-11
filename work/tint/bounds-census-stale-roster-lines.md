---
id: bounds-census-stale-roster-lines
kind: issue
title: bounds_census's roster keeps two stale lines (curves/boxes.rs lifted, surgery.rs boxed_reach) and is red on main
status: closed
opened: 2026-11-03
priority: P1
cost: E
closed: 2026-11-11
branch: tint/two-slow-reds
pr: 6818
---


Found by ONECOPY's frame-readings unit (PR 6463), which updated this census for its own sites.

`crates/geom-core/tests/bounds_census.rs` `every_sole_bracket_bound_door_is_in_the_roster` (slow set, nightly) is red on main:

- **arrived or moved**: `crates/geom/src/curves/boxes.rs` `lifted`. The roster lists it under the subject `fn lifted<T: Bounds>`, but the census now reads the item's subject as its name, `lifted`.
- **listed but no longer present**: `crates/geom/src/curves/boxes.rs` `fn lifted<T: Bounds>`, and `crates/sweep/src/blend/surgery.rs` `boxed_reach`. On main `boxed_reach` is bounded by `SpanLocate`, not `Bounds`.

Fix: re-point `lifted`'s line at its subject as the census reads it, keeping its disposition, and drop `boxed_reach`'s (or record why its new bound still owes a line).

## Dispatch

2026-11-11, TINT orchestrator. One implementer, lane `tint-reds`, branch `tint/two-slow-reds`, which takes both of TINT's open P1 slow-set reds together: the Debug-shape pin in `germ_tilted_rod` and the stale `bounds_census` roster lines. The lane re-measures each on main first. Review tier: the orchestrator's read, since both are test-side re-pins with no kernel change. If a fix has to land in `crates/*/src`, that becomes a style review, announced to the crate's owner.

## Closed (2026-11-11, PR 6818, by TINT)

Both lines this row names were already fixed on main before it was
dispatched:
- `lifted` was re-pointed at the subject the census reads in `5cf5c2fbc5`
  (2026-11-03).
- `boxed_reach`'s line was dropped in `11b103efcb` (2026-11-08). The census's
  docs agree with dropping it. It rosters sole-`Bounds` doors, and a
  `SpanLocate` door is outside its walk (its gap 5). Gap 5 records only a
  bracket read that is a selection moving with the inputs.
  `blend::surgery::boxed_reach` reads no bracket itself: its bound is there
  for `cover_over`, and it returns the cover's `T`-valued ranges.

Measured on main (`b291655`): still red, on a newer arrival.
`crates/geom-brep/src/edge_nurbs.rs` `knot_lines` (PCERT, `bcbbe4fe5c`,
merged by PR 6532) is a sole-`T: Bounds` door with no line, and nothing is
listed that is no longer present.

Its line is `Payload`:
- each interior knot run's bracket midpoint becomes the `f64` line where a
  bending carrier's General image breaks its `f64` path;
- which runs are lines is read off the held structure's multiplicities
  (`KnotValues::runs` reads `KnotStructure::multiplicities`), never off the
  bracket.

Green at ε 1e-9, 1e-6 and 1e-12.
