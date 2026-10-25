---
id: offset-surfaces-together-leaves-joinable-vertices-at-rest
kind: issue
title: offset_surfaces_together joins at most two of a cut edge's vertices and returns Ok with the rest joinable at rest
status: open
opened: 2026-10-24
priority: P2
cost: E
---

Found while pinning
`an-offset-doors-trailing-join-sweeps-tier-1-per-kill` (OFFSET,
2026-10-24).

Fixture: a unit brick with one edge cut by `k` vertices
(`edge_join::join_door::brick_with_splits`), every face in the move
set, the cut edge's two faces held at `0.0` and the other four moved
by `-0.1`. On it the two simultaneous doors disagree:

| k | `offset_planes_together` joins | `offset_surfaces_together` joins | left joinable after `offset_surfaces_together` |
|---|---|---|---|
| 1 | 1 | 1 | none |
| 2 | 2 | 2 | none |
| 3 | 3 | 2 | 1 |
| 4 | 4 | 2 | 2 |
| 5 | 5 | 2 | 3 |

(`joinable_vertices` on the returned body, which the door returns
`Ok`.)

Why: `offset_general.rs` (`offset_surfaces_together_staged`) hands the
moving charts alone to `replace_face::offset_charts_staged`, whose
join `within` is `ends` — the endpoints of the edges it `written`,
computed once before the join. Only the two end pieces of the cut edge
are written (their corner moves), so only the cut vertex next to each
corner is in `ends`; an interior cut vertex never is, even once the
join has made it the end of a joined (and re-described) edge.
`offset_planes_together` joins over `Scope::holds_vertex`, the whole
solid, and takes every one.

So whether a vertex joins depends on which door the caller picked for
an all-planar solid and on whether its neighbour joined first. Either
the general door joins over its whole scope, as its planar twin does
(`scope.holds_vertex`, which its own no-move path already uses), or
the narrowing is deliberate and the planar door's wider join is the
one to change; decide which, then pin both doors on this fixture.
