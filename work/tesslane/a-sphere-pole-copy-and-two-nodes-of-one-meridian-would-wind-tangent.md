---
id: a-sphere-pole-copy-and-two-nodes-of-one-meridian-would-wind-tangent
kind: issue
title: A pole copy and two lattice nodes of one meridian would make a resolved triangle whose normal is tangent to the sphere; unmeasured
status: open
opened: 2026-11-17
priority: P2
cost: M
---

## Finding

Raised by a designer on the fork of 2026-11-17 (row 279 of `docs/DESIGN-FORK-LOG.md`), by reading, not measured. The rectangle lane triangulates a sphere face in its raw chart, where a pole is a row of copies. A triangle on one pole copy and two lattice nodes of one meridian lies in that meridian's plane: it has area, so `validate::check_winding` calls it resolved, and its normal is tangent to the sphere rather than outward. The join reads resolution, not outwardness against the body.

The cone's like triangle (an apex copy and two nodes of one ruling) has no area and refuses `UnresolvedWinding` (`a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`). The sphere avoids the shape today only because a meridian side is an arc and is chorded at the row pitch, so the triangulation has a nearer point to take.

## Not measured

Whether any body makes the triangle. The designers' sphere cells (a ball and a sphere band with a split meridian, δ 0.1 to 0.001) mesh and pass the resolution check; none ran an outward check on them. A sweep of sphere faces through a pole with split and unsplit meridians, read by `reach_probes::geometry`, settles it.
