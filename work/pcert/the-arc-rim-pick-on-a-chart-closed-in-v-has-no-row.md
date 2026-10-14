---
id: the-arc-rim-pick-on-a-chart-closed-in-v-has-no-row
kind: issue
title: nothing exercises the arc rim's row-and-direction pick on a chart whose two boundary rows coincide in 3-D (closed in v), the one case where candidate order decides
status: open
opened: 2026-10-13
priority: P3
cost: E
---


Found by the review of PR 4988 (`pcert/arc-rim-row-pick`, its S6).

## What is missing

`nurbs_iso_derive`'s arc rim (`crates/topo/src/pcurves.rs`, the
`arc_rim` closure) offers four `IsoArc` candidates to `probe_pick`:
each boundary row `v ∈ {v₀, v₁}` × each `u` direction, `v₀` first. On a
chart closed in `v` the two rows are one 3-D curve, so both rows' images
hold the carrier at the probe and the pick returns the `v₀` one by
order alone. That is the one case where the candidate ORDER, not the
geometry, decides the stored row. Both answers lie on the same 3-D
locus, so the arc-rim certification accepts either; what is unpinned is
which row the loop walk then sees, and whether it closes.

## Why no row in 4988

A chart closed in `v` with the rim running along `u` is a torus-like
spline wall: the rim is a circle along `u`, and `v` closes on itself.
`sweep::loft_body`, the door 4988's rows use, stacks sections, so its
`v` is open; 4988 did not find another door that builds one, and did
not survey them all. Building it needs a spline chart closed in `v` set on a face whose
rim edge is a circle along `u`, through a door that keeps the rim's
description coherent.

## Fix

A row on such a body: the rim derives its image, the mint stores it,
and the loop walk closes over it. If the `v₀`-first order makes the
walk refuse where the `v₁` row would close, the pick wants the loop's
own context (the neighbouring rows), not a fixed order.
