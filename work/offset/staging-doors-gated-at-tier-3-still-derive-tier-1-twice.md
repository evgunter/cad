---
id: staging-doors-gated-at-tier-3-still-derive-tier-1-twice
kind: issue
title: The doors whose typed gate is tier 3 (the boolean, shell, the split, the STEP import) still derive tier 1 at their close and again inside the at-rest gate
status: open
opened: 2026-10-24
priority: P2
cost: M
---

The residue of `a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`
(OFFSET, 2026-10-24).

That unit gave a staging door one tier-1 derivation at its end,
`Surgery::close_gated` (`crates/topo/src/surgery.rs`): `closed_by_tier`
once, a tier-1 finding a debug panic naming the door, the tier-2 half
the door's typed refusal. It fits every door whose typed gate is tier 2
(the three simultaneous offset doors through `offset_charts_staged`,
`offset_planes_together_staged` and `offset_charts_together_staged`,
and `merge_coplanar_faces_staged`).

It does not fit a door whose typed gate is TIER 3, and those still
derive tier 1 twice per call:

- the boolean (`boolean::ops`, the finish: `body.sweep_and_close()`
  then `gate`, which runs `sort_into_pieces` on the body and then
  `AtRestPolicy::gate_at_rest_kept`, tier 3 whose first act is tiers 1
  and 2; at a dual, `structural_gate`'s `validate_closed`);
- `shell` (`shell.rs`, `out.sweep_and_close()`, then the join, then
  `validate_geometric`);
- the split (`splitting::finish`, the reassembled body's sweep, then
  `carve` into two sides, then `split_direct`'s per-side
  `validate_closed`, twice, around `end_maximal`);
- `step-import`'s `build_one_solid` (`door.sweep_and_close()`, then the
  join, then the import's at-rest gate).

In each the typed gate reads a body the door has changed since the
scope closed (pieces sorted, edges joined, sides carved), so the gate's
own tier 1 is not over the state the sweep checked, and folding the two
means either moving those steps inside the scope or letting the tier-3
gate carry D1's panic (a door name threaded into
`AtRestPolicy::gate_at_rest_kept` and its four scalar impls, or a tier
split of its findings). Either is a change to how D1's postcondition is
spelled at those doors; size it before building.
