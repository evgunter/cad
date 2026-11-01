---
id: one-home-for-the-clearance-witness-verifier
kind: issue
title: One home for the clearance witness verifier: the closest-station search by upper end is copied in shell.rs, editor-core and three tests
status: open
opened: 2026-11-01
priority: P3
cost: E
---


## Finding

Found by SHELL-4's review (PR #6183).

The verifier a clearance sweep's caller hands `Sweep::run` /
`ClearanceLane::sweep` has the same core everywhere:
1. take `Candidate::on_face_stations`;
2. evaluate every station pair on the two stored charts;
3. keep the closest pair (by upper end or by `f64` distance);
4. decide it at the bound's own funnel site (`ClearanceBound::predicate`,
   strict zero for `StrictlyPositive`).

That core has no home in `topo::clearance`. It is written out in five
places:
- `crates/topo/src/shell.rs` `cavity_embedded`'s closure (closest pair
  by upper end, on the charts lifted to `Interval`);
- `crates/editor-core/src/clearance.rs` `verify_witness` (closest by
  `f64` distance on an `f64` rebuild);
- `crates/sweep/tests/self_intersection_proves_injectivity.rs` `verify`;
- `crates/topo/tests/a_seamless_pipe_cut_by_an_oblique_plane_measures.rs`;
- `crates/sweep/tests/clearance_reads_an_iso_bounded_wall.rs`.

## What would close it

One door in `topo::clearance` that takes the two charts (or a lift),
the stations and the bound, and answers the witness or the reason it
could not confirm one. Every caller above becomes a call to it. The
editor-core verifier keeps its own `f64` rebuild of the leaf and hands
the door the rebuilt charts.

