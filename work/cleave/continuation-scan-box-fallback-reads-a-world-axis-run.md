---
id: continuation-scan-box-fallback-reads-a-world-axis-run
kind: issue
title: The undeclared-continuation scan's box fallback reads a long world-axis overlap of two ellipse or spline edge boxes as a shared curve, so its refusal depends on the pose
status: open
opened: 2026-10-06
priority: P1
cost: M
---


Found by the operand-gate pose lane
(`work/reach/boolean-operand-gate-separates-only-along-world-axes.md`)
sweeping `topo/src` for box overlaps read as a verdict. Unmeasured:
read off the code; no fixture has been built for it.

## What

`boolean::reduce::refuse_undeclared_continuations` asks whether two
faces of a same-oriented undeclared carrier pair meet along their
boundary, edge pair by edge pair, behind an edge-box overlap
(`bx.overlaps(&by)`). `edges_share_a_curve` decides it point-on-edge
through `Decide` wherever both carriers have a point parameter. For an
ellipse or a spline edge it falls back to the boxes: they share a
curve when the two padded world boxes overlap on every axis and run
past `POINT_TOUCH_RUN` pads on one. The run along a world axis is a
fact about the pose: two edges that touch at one point and leave it
along a direction off every axis overlap by more than four pads on the
axis they lean along, so the scan refuses an undeclared coincidence
that turning the pair would not.

## What would close it

A pose-free reading of "shares a curve" for the carriers with no point
parameter: the run measured along the edges' own direction (the chord
between an edge's ends), or a point parameter for the ellipse. The
refusal direction is unchanged either way (over-reporting a meeting
refuses).
