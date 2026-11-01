---
id: staging-doors-gated-at-tier-3-still-derive-tier-1-twice
kind: issue
title: The doors whose typed gate is tier 3 (the boolean, shell, the split, the STEP import), and the merge door with its re-description scope, still derive tier 1 twice per call
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
the door's typed refusal. It derives tier 1 once at the end of the
three simultaneous offset doors (through `offset_charts_staged`,
`offset_planes_together_staged` and `offset_charts_together_staged`).
`merge_coplanar_faces_staged` closes gated too, but it does not
derive tier 1 once; see the last section.

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

## The merge door

`merge_coplanar_faces_staged` (`crates/topo/src/merge_faces.rs`)
closes its run's scope with `close_gated`. Then, when any group
merged, it opens a second scope for `boolean::describe_edges` over the
kept faces' boundaries and closes that with `sweep_and_close`. That is
two whole-body tier-1 derivations per call.

Folding the describer into the gated scope is not sound as the door
stands:
- the describer walks the kept faces' loops (`boundary_edges`, which
  is `unreachable!` on a loop that does not walk), so it needs the
  gate's tier-1 and tier-2 verdict before it runs, or a torn result
  stops being reported typed;
- the edge descriptions it writes are a mutation that the sweep after
  it must re-check.

The way out is either a describer that refuses typed on a walk it
cannot make, so it can run before the one gate, or a local tier-1
proof over the edges it wrote in place of the second whole-body sweep.
