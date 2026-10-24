---
id: an-offset-doors-trailing-join-sweeps-tier-1-per-kill
kind: issue
title: An offset door's trailing join runs outside its surgery scope, so each kill sweeps tier 1 over the whole body
status: closed
opened: 2026-10-24
priority: P2
cost: E
closed: 2026-10-24
pr: 5666
branch: offset/trailing-join-one-sweep
---

Found while tracing `Scope`'s O(body) account for
`a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`
(OFFSET, 2026-10-24).

Every offset door that joins closes its surgery scope with
`close_gated` and then runs `replace_face::staged_join` on the staged
body:
- `offset_together.rs` (`work.close_gated("offset_planes_together")`,
  then `staged_join`)
- `offset_axial.rs` (`offset_charts_together`)
- `replace_face.rs` (`offset_charts_staged`)
- `offset_general.rs` (its `staged_join` call)

`staged_join` calls `Body::join_edges_within`
(`boolean/edge_join.rs`), whose loop `join_all` kills each joinable
vertex through `join_one` → `Body::kev_describing`
(`euler_kill.rs`). That ends in `assert_euler_postcondition` →
`assert_tier1_postcondition`, and at depth 0
(`tier1_sweep_is_mine`) that is a whole-body `validate`. So a door
that joins k vertices derives tier 1 k more times after its
`close_gated`, in every debug-assertion build, this workspace's
release profile included.

The n-gon prism bench (`topo::all ngon_prism_offset_scaling`) joins
nothing, so it does not show this. A body whose offset leaves k
collinear rim edges to join pays k × body.

Fix shape: put the join inside one scope closed with
`sweep_and_close`, so k kills cost one walk. The other order, joining
inside the door's own scope before `close_gated`, changes what the
gate reads (a refused join over a torn body), so weigh it before
taking it. The same shape probably applies to the split's halves,
`shell` and the blend, which also call `join_edges_within` on bodies
they staged; check whether their joins run inside a scope.

## Closed

PR 5666. `Body::join_edges_within` runs its kills and re-mint under
one surgery scope closed with `sweep_and_close`, so a join derives
tier 1 once (none where it joins nothing). Every caller is covered:
the offset doors through `staged_join`, `shell` and the blend.
`Body::join_edges` does the same on its staged clone, which covers the
merge, the split's halves and STEP import.
`edge_join::join_door::a_join_sweeps_tier_1_once_however_many_it_joins`
pins it with `validate::TIER1_DERIVATIONS`, constant in k. Residue
found on the way:
`offset-surfaces-together-leaves-joinable-vertices-at-rest`.
