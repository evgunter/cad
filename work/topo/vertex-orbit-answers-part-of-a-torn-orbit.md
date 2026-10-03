---
id: vertex-orbit-answers-part-of-a-torn-orbit
kind: issue
title: Body::vertex_orbit, the half-edge-keyed door, answers part of an orbit a torn next split or closed past some of its members
status: open
opened: 2026-10-03
priority: P4
cost: E
refs: [a-fan-split-at-a-vertex-another-vertexs-torn-walk-merges-into]
---


## What

`Body::orbit_inverts` (`crates/topo/src/body.rs`) proves a closed vertex
walk is the vertex's whole orbit on a body torn by `next` writes: each
member's inverse step `mate(prev(·))` is the member before it. The fan
split's plan (`Body::mev_fan_plan`, `euler.rs`) and every vertex-keyed
read (`Body::vertex_orbit_of`, so `edges_of_vertex`, `faces_of_vertex`
and their callers in `boolean`, `splitting`, `offset_axial` and
`shell`) refuse a walk that fails it.

`Body::vertex_orbit`, the public door keyed by a half-edge, does not.
Its docs say so: on a torn body its `Some` lists half-edges that all
start at the vertex, but not necessarily all of them. Its readers today:

- `kev_plan` (`euler_kill.rs`) reads the dying vertex's walk as a set
  and proves the set whole with `require_vertex_unnamed`, a whole-arena
  pass that names the first stranded half-edge. Putting the inversion
  proof inside `vertex_orbit` makes `kev_plan` refuse earlier with
  `OrbitBroken { he: m }` and lose that name: tried in the PR that
  filed this row, six kev rows in `euler_kill` and `review_d18` red on
  the refusal they pin.
- `boolean::sectors` (`along`, the sector edge reader) reads one step
  `orbit[1]` of an operand at rest, not the orbit as a set.

## The shape to give

Either put the proof inside `vertex_orbit` and move `kev_plan` onto a
walk that keeps naming the stranded half-edge, or keep the door as
documented and say why a half-edge-keyed read may answer part of an
orbit. No measured fault rides this today, so it is P4.
