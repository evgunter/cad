---
id: a-spline-implicit-arc-is-carried-to-its-node
kind: unit
title: A NURBS x implicit arc is carried to its node, certified by limb 3's graded tube
status: open
opened: 2026-11-12
priority: P1
cost: H
blocked_on: [the-join-reads-a-spline-implicit-section]
---


Split out of `the-join-reads-a-spline-implicit-section` by the SECTARMS
orchestrator (2026-11-12). Ev approved the design in PR 6829, fork row
262: "i'm not sure about the considerations here, but this sounds
broadly reasonable". It follows the join because both change the
carried component's pad (`CarriedComponent::pad`).

## What

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

Every arc of a spline × implicit node is carried to the node's point,
limb 3 certifies it at trace time and at rest, `ray_of`'s chord pick is
gone for spline nodes, and the nearly-osculating row is in.
