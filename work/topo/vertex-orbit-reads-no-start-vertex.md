---
id: vertex-orbit-reads-no-start-vertex
kind: issue
title: vertex_orbit's walk reads no start vertex, so every read-side orbit walk answers for a torn orbit as if it were the vertex's
status: closed
opened: 2026-09-29
refs: [mev-fan-plan-trusts-the-orbits-start-vertices, kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
priority: P3
cost: E
pr: 3972
branch: topo/vertex-orbit-proves-its-start
closed: 2026-10-03
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
| `classify_neighborhood` (`splitting/neighborhood.rs`) | sectors measured from this vertex's point along another vertex's edges; the fan insertion it feeds now refuses at `mev_null`, strut sites included |
| `build_sectors` (`boolean/sectors.rs`) | the same, for the boolean ON-set |
| `fan_edge_between`, `incident_faces`, and the two valence reads in the seam-run kill (`boolean/rest.rs`) | a wrong edge, face set or valence, which picks `kev` or `kemr` |
| `strut_tip` (`merge_faces.rs`) | a wrong valence-one answer, which picks `kev` or `kemr` |
| `corner_arms` (`offset_axial.rs`, `offset_together.rs`) | arm lengths along another vertex's edges |
| `valence` (`shell.rs`) | a wrong valence |
| `crates/sweep/src/blend/admit.rs` (`faces_of_vertex`, the admission's face fan) | another vertex's faces among the corner's |
| `crates/sweep/src/blend/battery.rs` (three `edges_of_vertex` reads: `cap_incidence`, `corner_at`, `joint_verdict`) | another vertex's edges among the corner's |
| `crates/sweep/src/blend/surgery.rs` (five `edges_of_vertex` reads: the chain-end corners, three corner and rim fans, and the cap's meridian split) | the same; the last picks the meridian edge the surgery then splits, so it plans a write |

The first sweep, for the symbols `vertex_orbit(` and `orbit_walk(`,
could not see the sweep rows: they reach the walk through the public
`Body::edges_of_vertex` / `faces_of_vertex`, which call
`orbit_projection`. They came from a second pass for those two names
over `crates/`, `demos/` and `tools/`; outside `crates/topo` it found
production callers only in `crates/sweep/src/blend/`, and the rest are
tests. `work.py territory` puts those files in `band`'s and `carve`'s
ground; the row stays here because the walk is topo's.

The validator's pass 6 (`validate.rs`, through `orbit_walk`) is not in
this class: it reads every member's start itself and reports
`OrbitForeignMember`.

A clean walk from `he1` is not enough on a torn body either. In the
fix pass's tear search (`review_d18`'s fixtures, seeds 1..=3,000 at
one and two `NextForeign` tears), 975 of 1,101,856 non-strut
`mev_null` calls that returned `Ok` on a walk that stays at `v` put a
minted key in an orbit error in the result's `validate`, because
another vertex's rho-shaped torn walk merges into `v`'s cycle. Neither the whole-walk
check in `mev_fan_plan` nor the `orbit_walk` proposal below sees it,
since the walk from `he1` never leaves `v`; a plan would have to prove
the orbits that end in `v`'s, not only `v`'s own.

## The shape to give

Either each caller proves its walk as the two plans do, or the walk
proves it once: `orbit_walk` answers `Broken` for a member whose start
differs from the first member's. The second covers every caller and
makes the two plan checks redundant. It changes what `orbit_walk`
returns on a torn body, and pass 6 reads the `Closed` members to name
each foreign one, so the validator would need its own walk or a
`Walk` arm that carries them.

## Resolution (PR 3972)

The walk proves the start once: `Body::orbit_walk` answers `Broken`
at a member that does not start at the first member's vertex. The
validator's pass 6 walks `Body::orbit_walk_reading_no_start` and names
each foreign member as it did before. Measured over 1,056,000 torn
bodies, its reports are identical at base and head. `kev_plan`'s and
`mev_fan_plan`'s own checks are retired. A torn fan walk now refuses
`FanOrbitBroken`. `shell`'s `valence` refuses a broken orbit instead of
answering 0.

The rho residue is re-measured at head (965 of 1,103,470 `Ok` calls)
and filed as `a-fan-split-at-a-vertex-another-vertexs-torn-walk-merges-into`.
The receipt filed two rows on cleave:
`null-site-reads-a-refused-vertex-orbit-as-no-edges` and
`boolean-strut-anchor-splices-at-an-unproven-next-mate-step`.
