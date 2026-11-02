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

## Done when

The join builds a sphere × lofted-wall union through the value, and the NURBS wall's plane-wall lane is gone.
