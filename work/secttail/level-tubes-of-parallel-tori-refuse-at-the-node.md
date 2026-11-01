---
id: level-tubes-of-parallel-tori-refuse-at-the-node
kind: issue
title: Two parallel-axis tori whose tube tops or bottoms stand level refuse R-tan: the parallel-tori arm does not read its loops through the node
status: closed
opened: 2026-10-27
priority: P2
cost: M
closed: 2026-10-28
---


Found by SECT's lily lane (`sect/lily-parallel-tori-certificate`), building
the parallel-tori arm, 2026-10-27.

## Measured

`boolean/section_cert.rs` `torus_parallel_torus` decides which tube owns
the top and the bottom of the heights both tubes span
(`section_torus_parallel_tori_top`, `..._bottom`). Where the two tops (or
bottoms) stand level — `h + r₂ = r`, as for two equal tori side by side
at one height — the matching curve `W` of the two meridian circles'
heights has a node there (both tubes' sheets turn at once), and the arm
refuses R-tan rather than read its loops through it. Pinned by
`section_cert_parallel_tori_rows::level_tubes_of_one_radius_refuse_as_a_tangency`
(the donut `R 2, r 0.5` against its copy offset `1` along `x`).

Equal tubes at one height are an ordinary pose (two copies of one ring
part, side by side), so the refusal is reachable on normal geometry.

## What the taker owes

Read `W` through the node: at a level top the four strands meet at one
point, and the loops' pairing there follows from which strands carry the
triangle `|ρ − ρ₂| < e < ρ + ρ₂` either side (the point is generically off
`Π`, so the section is smooth through it or the carriers are tangent).
Pin it against the trace the rows already carry.

## Closed

2026-10-28, by SECT's `sect/two-tori-subtract-red`. `torus_parallel_torus`
now reads past a level end whose node `(R, R₂)` clears the strict triangle
`|ρ − ρ₂| < e < ρ + ρ₂` by more than the neck `√(2·max(r, r₂)·δ)`. There
every reading of the end cuts the same arcs inside the triangle. Where the
node sits in the triangle or on its edge, the level ends are points of both
tori with both normals along the axes, a real tangency, and the arm still
refuses R-tan. The row pinned here (offset `1`, node inside) keeps refusing
as `section_cert_parallel_tori_rows::level_ends_with_the_node_inside_refuse_as_a_tangency`.
`parallel_tori_every_class_against_the_trace` holds four new rows clear of
the triangle against the trace.
