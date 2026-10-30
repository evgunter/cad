---
id: the-area-pass-at-32-cells-a-span-halves-a-one-span-faces-resolution
kind: issue
title: the area pass at 32 cells a knot span halves a one-span face's area resolution
status: open
opened: 2026-10-30
---


(QUAD per-span lane, `quad/per-span-round-zero`.)

## What

The rectangle lanes' area pass (`area_midpoint_taylor`, `crates/geom-brep/src/props/quad/area.rs`) is an O(h) rule at a fixed cell count, built once per face. It was 64 cells per axis over the whole rectangle; it is now `AREA_CELLS_PER_SPAN = 32` per knot span. A many-span patch gets far more area cells than it did (the 9 × 9-span conic torus's area pad went from 29.1 m² to 9.3 m²), and a one-span face gets half: `m6_loft_body`'s walls read 0.3425 m² where they read 0.1896 m², and the extreme-weight hypar's area midpoint moved enough to move its meter pin (`review_r1_rational_probes`' `HYPAR_FLOOR`).

## Why 32 and not 64

At 64 cells a span the whole torus pays 81 × 4096 area cells a wall per entry, about 5.6 s, and its at-rest gate (`gate3`) about 6 s against main's 1.86 s. At 32 the gate is 1.7 s.

## Owed

A higher-order area rule (the flux's Taylor remainder has one; the area's Lipschitz pad is first order because `|S_u×S_v|` kinks where the cross product vanishes), or a cheaper area cell (`Ladder::cross_num_at` collapses six grids' three channels each; the weight net needs one), would buy the one-span resolution back without the cost.

## The hull blocks, the same trade

`HULL_BLOCKS_PER_SPAN = 4` (`cells.rs`): a one-span face had 8 blocks a direction and has 4, so its Taylor-remainder hulls and its area pads are read over wider blocks. At the work wall (round 7 on one span cell) the composite arm's one-span carriers are about 2× wider than they were: the R = 2 m quarter torus 4.687e-8 → 9.301e-8 m, the unit sphere octant 2.920e-8 → 5.794e-8 m (`review_r1_rational_probes`' pins, ε = 1e-12). Both still certify the 1.024e-6 m default target, by 11× and 18×. At 2 blocks a span the whole 9 × 9 torus's at-rest gate costs 1.7 s against 2.6 s at 4 (main: 1.86 s), but the one-span carriers are 3.5× wider, and the extreme-weight hypar's area width (38 against a 6 m perimeter) trips the A2 tripwire at ε = 1e-6, a release panic; at 8, about 5 s.
