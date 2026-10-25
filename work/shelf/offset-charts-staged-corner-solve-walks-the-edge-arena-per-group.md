---
id: offset-charts-staged-corner-solve-walks-the-edge-arena-per-group
kind: issue
title: offset_charts_staged's corner solve walks every edge of the body per vertex group, and finds plans, charts and solved roots by linear search
status: open
opened: 2026-10-24
priority: P2
cost: E
---

Found by OFFSET's `a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`
(2026-10-24) in its sweep of the door files for linear finds over the
scope; filed here because SHELF's `offset-doors-take-solids-and-a-per-chart-rule`
rebuilds these doors' decide walks.

In `crates/topo/src/replace_face.rs`, under `offset_charts_staged`
(`replace_faces_offset`, `offset_surfaces_together`):

- `incident_edges` walks `body.edges()` once per vertex GROUP the corner
  solve visits (`solve_corners`, and `planned_charts`' copy), so the
  solve is O(vertices × edges) on its own. Driving it by the group's
  orbits is the obvious fix, but the whole walk is robust to a vertex
  whose half-edges form two fans and an orbit walk is not, and the
  result's order (edge-arena) feeds the solve, so it needs an arena
  rank, not a set.
- In the same function, `plans.iter().find(|p| p.edge == edge)` per
  incident boundary edge.
- `plan_reanchors`: per edge, `corners.groups … .find(group.contains)`
  and two `solved.iter().find` (the boundary test and the moved-point
  lookup there are indexed now).
- `moved_of(charts, key)`: a linear find over the charts, per edge in
  `plan_edge` and per vertex in the corner code — O(edges × charts)
  when every face of a solid moves.

Already indexed by the unit that found these: the boundary dedupe (both
copies), `read_ends`' moved-point lookup, `move_points_then_rechart`'s
staled-row filter, `group_by_point`. `split_by_solid`'s find is over the
solids sharing one point and runs only on a refused joint solve; left.
