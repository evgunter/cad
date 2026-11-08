---
id: recl-fan-on-guard-has-no-reaching-row
kind: issue
title: recl's VertexPairAtConeApex guard for a fan bound left On has no reaching row
status: open
opened: 2026-11-03
priority: P3
cost: C
---


Found by the delta review of PR 6424 (SPLITDOOR unit B stage B2).

## What

`boolean::recl::recl_edges`' postcondition
(`crates/topo/src/boolean/recl.rs`, "Postcondition: no surviving On
codes") refuses `VertexPairAtConeApex` where a surviving record holds a
fan sector (a sector at a cone's apex) with a bound `On` the partner's
face: the edge rules above it settle only a planar sector's `On` bound.

B1's fix pass added it for the 80° cone's `x90` box. B2's fix pass
found that pose, and a 30° wrap-tipped cone against a box edge along its
axis, reached it only through the apex corner's quarters collapsing onto
the seam (`boolean::sectors::push_apex_corner`, now cut by the window
the corner's shape decides). Since then no row reaches it: a panic in
its place leaves every apex, cone, fan, recl and sectors row green.

## Done when

One of:

- a pose that leaves a fan bound `On` a partner's face after the edge
  rules is found and pinned (the guard is then the refusal of record,
  and its row names it), or
- the edge rules are shown to settle every fan bound they can be handed
  (a fan's bound is a ruling, a straight line on the cone, so the
  planar rule's line argument may carry over), and the guard becomes a
  `ClassificationInvariant` (or is retired) with that argument in its
  comment.
