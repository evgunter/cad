---
id: props-sphere-side-in-arc-is-levered-by-arc-length-not-the-roots-slope
kind: issue
title: props: the sphere side's in-arc reading is levered by arc length, so near a graze a root within its own error of a span end is read decided
status: open
opened: 2026-10-08
---


(TANG implementer, the in-span class sweep of PR 4246's fourth review.)

## What

`side_on_meridian` (`crates/geom-brep/src/props/curved.rs:2736`) finds
an edge circle's two crossings of the meridian plane where
`a·cos t + b·sin t = −d`. It decides the meeting on the gap
`√(a² + b²) − |d|` (`props_sphere_side_roots`, `curved.rs:2796`), then
reads each root inside the edge's span by the parameter gap levered by
the circle's radius, which is arc length (`props_sphere_side_in_arc`,
`curved.rs:2813` and `:2819`).

Near a graze the root is ill-conditioned. A carrier moved by δ moves
the root by δ over the slope the circle crosses the plane at,
`|√(a² + b²)·sin ω|/ρ`, where ω is the root's half-angle from the
extreme. With the gap decided at `g ≥ Kε`, the root error reaches
`≈ δ·√(ρ/(2g))`, which is far past the band. So an arc-length margin
can be decided while the root lies within its own error of a span end.
That end is a vertex of the loop, where the meridian crosses one edge
or its neighbour. A root that is really at that vertex is then counted
on the wrong edge, or twice, and `props_sphere_side_order` and
`props_sphere_side_enter` read the wrong crossing.

The ring lane had the same shape (`topo::ring_path::path_parity`,
PR 4211's `sphere_path_parity` on main), and PR 4246 fixes it there.
Its review-3 probe found 722 wrong parities at ε 1e-12 under 4-ulp
carrier jitter, and its rows
`ring_path::graze_rows::a_graze_at_a_smooth_vertex_never_decides_a_wrong_parity*`
are the fixture to port.

## What it costs

`sphere_circle_loop_side` is a cross-check on a sphere face's material
bit, and an unanswered meridian checks nothing. A wrong answer is a
false contradiction of a correct bit (a refused face) or a missed
contradiction of a wrong one. Nothing is known to reach it.

## Fix

Lever each in-arc reading by the slope as well: the margin
`(s − t₀)·ρ·|n̂·τ̂|`, with `τ̂` the circle's unit tangent at the root and
`n̂` the meridian plane's normal `k`. That margin is the plane
displacement that moves the root past the end, which is the shape
`split_ring_path_in_span` now has. Then update the `props_sphere_side_in_arc`
row in `docs/predicate-dimension-audit.md:399`.
