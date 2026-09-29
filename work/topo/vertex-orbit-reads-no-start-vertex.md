---
id: vertex-orbit-reads-no-start-vertex
kind: issue
title: vertex_orbit's walk reads no start vertex, so every read-side orbit walk answers for a torn orbit as if it were the vertex's
status: open
opened: 2026-09-29
refs: [mev-fan-plan-trusts-the-orbits-start-vertices, kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
priority: P3
cost: E
---

## What

Found by the receipt of `mev-fan-plan-trusts-the-orbits-start-vertices`.

`Body::orbit_walk` (`crates/topo/src/body.rs`), behind
`Body::vertex_orbit`, steps `next(mate(·))` and closes when it
returns to its first half-edge. It reads no start vertex. A torn
`next` can close the walk through another vertex's half-edges, and
the walk reports it `Closed` as that vertex's orbit. Two write plans
now prove the walk they take: `kev_plan` (PR 3161) and
`mev_fan_plan` (the unit above). The read-side callers do not, and
each answers for the torn walk as if it were the vertex's. None
panics on it.

| caller | what the torn walk makes it answer |
| --- | --- |
| `Body::edges_of_vertex` / `faces_of_vertex` (`orbit_projection`, `body.rs`) | another vertex's edges and faces among this one's |
| `classify_neighborhood` (`splitting/neighborhood.rs`) | sectors measured from this vertex's point along another vertex's edges; the fan insertion it feeds now refuses at `mev_null`, a strut site does not walk |
| `build_sectors` (`boolean/sectors.rs`) | the same, for the boolean ON-set |
| `fan_edge_between`, `incident_faces`, and the two valence reads in the seam-run kill (`boolean/rest.rs`) | a wrong edge, face set or valence, which picks `kev` or `kemr` |
| `strut_tip` (`merge_faces.rs`) | a wrong valence-one answer, which picks `kev` or `kemr` |
| `corner_arms` (`offset_axial.rs`, `offset_together.rs`) | arm lengths along another vertex's edges |
| `valence` (`shell.rs`) | a wrong valence |

The validator's pass 6 (`validate.rs`, through `orbit_walk`) is not in
this class: it reads every member's start itself and reports
`OrbitForeignMember`.

## The shape to give

Either each caller proves its walk as the two plans do, or the walk
proves it once: `orbit_walk` answers `Broken` for a member whose start
differs from the first member's. The second covers every caller and
makes the two plan checks redundant. It changes what `orbit_walk`
returns on a torn body, and pass 6 reads the `Closed` members to name
each foreign one, so the validator would need its own walk or a
`Walk` arm that carries them.
