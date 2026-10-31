---
id: cylinder-and-torus-edge-misses-are-read-back-about-the-axis
kind: issue
title: The cylinder's and torus's per-edge whole-turn misses are read back about the axis, so their Interval width is charged as disagreement on a far generic-pose body
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by the FLUX 1e-12 main-reds unit (PR 5400).

The cone's `props_chart_loop_whole` escalated a legal ring and a steep wide frustum at ε 1e-12. Each cone edge's per-edge miss (`closure::EdgeTurn`, `crates/geom-brep/src/props/closure.rs`) was read back about the surface axis against the edge's own ends. At the interval scalar that read is about 1e-12 wide per edge, even where the true miss is exactly 0 (a generator, an ellipse's thirds) or O(tilt²) (a rim). The sum over the loop then passed the 3ε slack.

PR 5400 fixed the cone:
- `EdgeTurn::exact` for edges that agree with their ends by construction;
- `EdgeTurn::carried` for rims, whose miss is read in the rim's own frame, where the carry cancels.

The cylinder (`cylinder_windings`) and the torus (`torus_chart_loop`) still read every edge's miss back about the axis. No row is red today. The same width grows with the body's distance from the origin, though, so a far generic-pose cylinder or torus face with many edges could escalate the same way.

What closes it: give the cylinder and the torus the cone's two constructors (rulings and meridians exact; rims and parallels carried). Add a row: a far, generic-pose, many-edged face at 1e-12 that the old reading escalates and the new one measures.
