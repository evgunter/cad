---
id: level-tubes-of-parallel-tori-refuse-at-the-node
kind: issue
title: Two parallel-axis tori whose tube tops or bottoms stand level refuse R-tan: the parallel-tori arm does not read its loops through the node
status: open
opened: 2026-10-27
priority: P2
cost: M
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
