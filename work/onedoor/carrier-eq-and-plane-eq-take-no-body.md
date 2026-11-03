---
id: carrier-eq-and-plane-eq-take-no-body
kind: issue
title: "[topo.homes] arm 1: boolean/carrier_eq and boolean/plane_eq take no Body and stay in topo"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep` (2026-11-03), which checked them item by item and did not move them.

Item by item, `crates/topo/src/boolean/carrier_eq.rs` and `plane_eq.rs` take no `Body` and no arena key: `carrier_eq`, `carrier_eq_reading`, `pair_door_reading`, `CarrierRelation`, `CarrierEqError`, `CoincidenceMeasure`, `CarrierDesc`, `ConsumedExtent`, `INVENTORY`, `in_inventory`, `OUTSIDE_REST_INVENTORY`, `exact_curved`, the private rungs; and `plane_eq`'s `PlaneDesc`, `PlaneRung`, `plane_ladder`, `oriented_plane_eq`. Only the tests at the end of `carrier_eq.rs` build a `Body` (`unit_cyl_sheet`, through `carrier_pair::carrier_pair_relation`); they stay.

Why they were left: `onedoor/one-surface-record` is editing `boolean/carrier_pair.rs` (`one_carrier_at_rest`, which reads `carrier_eq`) in flight; the two are public API through `pncad`'s prelude and roughly 35 files read them; `plane_eq` re-exports `CarrierRelation as PlaneRelation` and `CarrierEqError as PlaneEqError`, which `engine-leaves` pins as refused bindings; and its four `bool_plane_*` K names (`bool_plane_parallel`, `_orient`, `_offset`, `_reach`) would take the module's prefix, a census note of their own. The move is one unit once `one-surface-record` lands: `git mv` both into `geom-brep` at its root, keep every signature, re-export the public names from `topo::lib`.
