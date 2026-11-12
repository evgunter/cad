---
id: carrier-eq-and-plane-eq-take-no-body
kind: unit
title: "[topo.homes] arm 1: boolean/carrier_eq and boolean/plane_eq take no Body and stay in topo"
status: spec
blocked_on: [face-pair-contact-pairing-has-four-spellings]
opened: 2026-11-03
priority: P1
---


Found by `crossing-doors-leave-topo-for-geom-brep` (2026-11-03), which checked them item by item and did not move them.

Item by item, `crates/topo/src/boolean/carrier_eq.rs` and `plane_eq.rs` take no `Body` and no arena key: `carrier_eq`, `carrier_eq_reading`, `pair_door_reading`, `CarrierRelation`, `CarrierEqError`, `CoincidenceMeasure`, `CarrierDesc`, `ConsumedExtent`, `INVENTORY`, `in_inventory`, `OUTSIDE_REST_INVENTORY`, `exact_curved`, the private rungs; and `plane_eq`'s `PlaneDesc`, `PlaneRung`, `plane_ladder`, `oriented_plane_eq`. Only the tests at the end of `carrier_eq.rs` build a `Body` (`unit_cyl_sheet`, through `carrier_pair::carrier_pair_relation`); they stay.

Why they were left: `onedoor/one-surface-record` is editing `boolean/carrier_pair.rs` (`one_carrier_at_rest`, which reads `carrier_eq`) in flight; the two are public API through `pncad`'s prelude and roughly 35 files read them; `plane_eq` re-exports `CarrierRelation as PlaneRelation` and `CarrierEqError as PlaneEqError`, which `engine-leaves` pins as refused bindings; and its four `bool_plane_*` K names (`bool_plane_parallel`, `_orient`, `_offset`, `_reach`) would take the module's prefix, a census note of their own. The move is one unit once `one-surface-record` lands: `git mv` both into `geom-brep` at its root, keep every signature, re-export the public names from `topo::lib`.

## Spec (ONEDOOR, 2026-11-12)

`one-surface-record` has landed, so the reason the move was held is gone. The final state:

- **One home.** `carrier_eq.rs` and `plane_eq.rs` move to `geom-brep`'s root with every signature kept, because they take no `Body` and no arena key (`[topo.homes]` arm 1). The tests that build a `Body` stay in topo, beside `carrier_pair`.
- **One name per type.** `PlaneRelation` (an alias of `CarrierRelation`) and `PlaneEqError` (an alias of `CarrierEqError`) retire, and every reader names the one type. Two names for one fact is the kind of second place this program exists to remove. `engine-leaves` already pins them as refused bindings, so its pin falls.
- **One path.** There is no re-export from `topo::lib` and no alias left behind: readers, `pncad`'s prelude, the python stubs, `demos/`, `tools/` and `benches/` all import from `geom_brep`. This is the rule the earlier path moves were reviewed against, so the old path cannot quietly stay the one people copy.
- **K names.** The four `bool_plane_*` names take the prefix of their new home, under the K-naming rule, with a roster-change entry in `docs/K-REPORT.md`. Do this only if the rule asks for it; say which way and why.
- **Unblocks** `S173`: `face_carrier` follows `CarrierDesc` out of `boolean/`.
- **Letter S, tier dual.** It is a move with no new decision, but it moves public API that `pncad` and about 35 files read, which the plan's standing rule makes a dual-tier unit. Under the protocol, S means sequential reviews.
- **Dispatch after PR 6890 merges**, because 6890 edits `carrier_eq.rs` (`CarrierEqError::reported`) and the census.

