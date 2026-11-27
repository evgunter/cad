---
id: a-section-circles-ring-vertex-beside-a-chart-pole-is-unread-and-one-sphere-op-loses-its-mesh
kind: issue
title: a section circle's ring vertex placed beside a sphere's chart pole is unread by the ring door, and one op that meshed loses its mesh
status: open
opened: 2026-11-26
priority: P2
cost: M
refs: [an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader, a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm]
---

Found by GERM U4b (branch `germ/u4b-face-reading`), the slow set's
`spheres_crossing_off_every_edge` rows.

## What it is

The sphere extent scan cuts the face holding a whole section circle
along its chart's meridian through the circle (`boolean::ops`,
`sphere_extent_scan`, `crates/topo/src/boolean/ops.rs:6558`). That
meridian edge crosses the other sphere twice. In
`a_trimmed_face_cut_beside_its_pole_builds` at δ = 10⁻³ the crossings
are at `(0, 0.99978, 0.021)` and `(0, 0.9999995, 0.001)`. The second
is 10⁻³ from the unit ball's chart pole `(0, 1, 0)`.

The crossing reading returns the first crossing the face holds
(`reduce::wall_crossing`, `crates/topo/src/boolean/reduce.rs:5609`),
so the edge splits there first, and the section circle's one ring
vertex lands on that crossing:
- on main, the root door's own emission order put the far crossing
  (`t` = 3.1406) first;
- since U4b, `cross`'s meetings are read in span order (ascending
  `t`), which puts the crossing beside the pole first.

Both bodies are valid, with identical topology in all 36 ops of
`a_trimmed_face_cut_beside_its_pole_builds` and
`a_circle_beside_a_pole_builds_though_the_mesher_refuses_it`, and
each op's volume holds to 1e-9.

A ring whose vertex sits beside the chart pole is not cleared by the
ring door's analytic reader (`pcurves_rings.rs`, `Outer::side`). It
answers `RingPlacementError::Undecided`, so the mesher refuses the face
(`TessellateError::DomainUnread`), which is the limitation filed in
`an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader`.

What it costs:
- `trimmed, δ 10⁻³, a ∖ b` meshed on main and now refuses at the ring;
- at δ = 10⁻⁵, the `a ∪ b` and `a ∖ b` ops of both rows refused
  `FoldedTriangle` beside the pole
  (`a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm`) and
  now refuse at the ring first, so no row witnesses the fold any more.

## What fixes it

Either:
- **the ring reader clears a ring point beside a chart pole**: the
  existing ring-reader row's tightening boxes, or a reading of the
  point that does not go through the pole's degenerate chart rows; or
- **the scan places a ring's vertex away from a chart pole**: the
  cut's crossing chosen by distance from the pole rather than by
  span order. That is a choice in the boolean, not a fix to the
  reader, and it hides the reader's limit rather than lifting it.

The first is the fix. The rows' wants record the ring refusal until it
lands, and a meshed `trimmed, δ 10⁻³, a ∖ b` is the evidence it has.


## 2026-11-27: on the one-band full revolve (PR 7414)

On `lathe/full-revolve-one-band` merged with main at `1d33364737` (U4b's span-ordered meetings included), the ring reader places these rings. `a_circle_beside_a_pole_builds_though_the_mesher_refuses_it` and `a_trimmed_face_cut_beside_its_pole_builds` pass with every `10⁻³` op drawing; at `10⁻⁵` the two unions and `a ∖ b` refuse at a triangle folded over the chord beside the pole (`FoldedTriangle`, `work/tesslane/a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm.md`), and the rest draw. No row there reads `DomainUnread { RingPlacementError::Undecided }`. The rows pin that reading, so the ring refusal this issue records does not reach a pinned row once PR 7414 lands.
