---
id: plane-section-polygons-drop-their-arcs
kind: issue
title: plane_section reports a section polygon by its corners only, so an arc-edged section (a bore's circle: two corners) has no recoverable shape or area
status: open
opened: 2026-10-01
priority: P2
cost: M
branch: cleave/section-arcs
---


Found by the section-ccw lane (`cleave/section-ccw`), measuring
`plane_section` on the bored fixtures of
`crates/sweep/tests/split_section_rings.rs`.

**Measurement.** `SectionPolygon` (`crates/topo/src/splitting/section.rs`)
carries `points` and `uv`: the corners of the section loop
(`loop_points_of`), nothing about the edges between them. On
`bored_brick(0, 0, 1)` cut at `z = 1.25`, the bore's polygon comes back
with two corners and shoelace area `0`; on
`bored_cylinder(0.3, 0.2, 0.37)` cut at `z = 0.5` both polygons (the
outline circle and the bore) have two corners and area `0`. Straight
edges are unaffected: the brick's outline at tilt 0.3 reads
`16 / cos 0.3 = 16.748…`, as it should.

**Why it matters.** A consumer of the query cannot draw, offset or
measure any section with a curved edge: the circle is two points. The
region roles are right (the winding that decides them is read on the
carriers, `section_sense`), but the polygon a role is attached to does
not describe its boundary.

**Fix shape.** Report each edge of the polygon with its carrier (line,
circle or ellipse arc in the section plane, in `(u, v)`), or a
certified sampling of it; keep the corners as the edges' ends.

SHOW `projectbox` (demos/tour/src/cutaway.rs, `read_section`): the
tour's section read-back of a bored enclosure cannot measure its bore
holes or its round bosses' outlines from `plane_section` (each is two
corners, shoelace 0), so the scene supplies πr²/cos φ in closed form.
