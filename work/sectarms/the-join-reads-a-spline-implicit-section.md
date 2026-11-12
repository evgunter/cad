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

## Open design questions (from NURBS stage 3b, 2026-11-04)

Stage 3b lists a NURBS × implicit section's nodes. Each arc ends on
entering its node's Krawczyk box (`RectEnd::Node` in the march), and
names at listing the ray it leaves the node along. Its carrier is still
unminted. Carrying such an arc, so that a chord from a node vertex has a
carrier reaching it, leaves two questions open for this unit:

1. **The carrier's end at the node.** The marched states stop at the box
   entry, not at the node. One option is to fit the carrier with its end
   point pinned to the node and its end tangent to the named ray's
   chart direction (a Hermite end condition, as at a rim). The other is
   to carry the arc only to the box and leave the node unreached.
2. **What certifies the stretch from the box entry to the node.**
   Limb 3's chart tube cannot read it, because `∇φ` tends to zero at the
   node. One option is the node box's own claim: Krawczyk uniqueness of
   `∇φ`'s zero, `det H < 0` over the box, and the node's point enclosure
   within the band. The other is for the arc to stop at the box, so that
   no carrier stretch inside it needs a certificate.

Both are written neutrally on purpose: neither option is recommended
here.

## Done when

The join builds a sphere × lofted-wall union through the value, and the NURBS wall's plane-wall lane is gone.
