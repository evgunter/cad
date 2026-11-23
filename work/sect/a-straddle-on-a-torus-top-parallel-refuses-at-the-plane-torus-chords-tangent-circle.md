---
id: a-straddle-on-a-torus-top-parallel-refuses-at-the-plane-torus-chords-tangent-circle
kind: issue
title: A straddle whose vertex sweeps a torus's top parallel refuses at the plane × torus chord's tangent circle
status: open
opened: 2026-11-20
priority: P3
cost: M
---

Found by the closed-edge-on-torus lane (`sect/closed-edge-on-torus`),
2026-11-20.

## What

The torus `R = 2, r = 1` revolved about `y` from two semicircles,
against the triangle `(2, 1), (2.3, 0.7), (2.3, 1.3)` revolved about the
same axis. Its vertex sweeps the torus's top parallel, its two cones
cross the tube there, and its cylinder `ρ = 2.3` crosses the tube at
`y = √0.91`. Every op in both orders refuses, at the default ε:

`Join(BooleanJoin(SectionInvariant { face, what: "tangent plane×torus
germ pair under a minted chord — a touching configuration, the typed
frontier of the supported envelope" }))`, raised in `chord_join.rs`
(the plane × torus arm of the chord's section reading, at
`PlaneTorusSection::TangentCircle`).

Before `closes_at_its_site` admitted a closed edge on a wrap edge, the
same pose refused `SingleSiteSectionLoop { count: 2 }`. The other
parallels (tube angles 0, ±π/3, 2π/3, π) build, and are pinned in
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_closed_edge_on_a_torus_parallel_holds_every_op`.

The circle's plane is tangent to the torus along the top parallel, so
the plane × torus classification reads the tangent circle the reading
refuses. Not measured at 1e-6 or 1e-12. Whether the chord should be
read on that plane at all is the question. The closed edge's own
section is the cone × torus pair's, and the plane is only the conic's.
The overlap's closed form, by Pappus: the region
`(2, 1) → (2.3, 0.7) → (2.3, √0.91)`, back round the tube.
