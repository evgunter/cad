---
id: the-join-reads-a-spline-implicit-section
kind: unit
title: The join reads a NURBS x implicit section: open components carried, chart pads in metres, GermLane::PlaneWall retired for a NURBS wall
status: parked
opened: 2026-11-02
priority: P0
cost: H
blocked_on: [a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm]
---


Filed by the SECTARMS orchestrator (2026-11-02) from the NURBS arm's phase-0 plan. The arm's four stages build the classification and the whole reading on the knot rectangle. None of them makes the boolean's join read the result.

## What

Once `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm` lands its whole reading (stage 3), the join still has to read a NURBS × implicit section through the value:
- carry open components with rim ends;
- convert the chart's pads to metres by the chart speeds (`pad_of`);
- match sites by foot along an open component;
- retire `GermLane::PlaneWall`/`WallPlane` for a NURBS wall (`topo/src/boolean/join.rs`).

This is what makes teapot wall 3 join once SPLICE clears its seam split. Tier: dual.

## The node stretch

Stage 3b (PR 6811) ends each arc's march on entering its node's grown
Krawczyk box (`Critical::region`, up to 1/16 of the patch wide) and names
the ray it leaves along. This unit carries every such arc on to its node,
as `crates/geom-brep/README.md` C2 (limb 3 at a node end) and C5 (the
spline × implicit node listing) state:

- the carrier's span ends at the node's point, with the named ray as
  its end tangent; samples inside the box are settled onto the section;
- limb 3's chart tube reads its radius per box, narrowing toward the
  node; its last box holds the node, with the node's own reading as
  its claim, re-read about the edge's end vertex at rest;
- the ray is the sector holding the chain's last boxes, checked against
  the end tangent, so `ray_of`'s chord-and-45° pick retires;
- the pad a site reads is per box; a site within about `√(εR)` of the
  node, and not at it, escalates.

Known costs: `chart_tube_windows` takes one uniform pad and `one_arc` has
no end arm for a chain ending at a node box; the per-box pad reaches
`SsiTube`, `Tube::of` and `CarriedComponent::pad`, and so the cylinder
and torus carried lanes' types. Add a named row for a nearly osculating
node (a 1 m sphere against a wall whose curvatures at the node are within
1% of the sphere's). If it fails on enclosure width, the remedy is `∇φ`
read through φ's Hessian (as `sweep/critical.rs::read` forms it) inside
the same test. Rows for `two-node-reading-arms-no-row-reaches.md` land
with this unit.

## Done when

The join builds a sphere × lofted-wall union through the value, and the NURBS wall's plane-wall lane is gone.
