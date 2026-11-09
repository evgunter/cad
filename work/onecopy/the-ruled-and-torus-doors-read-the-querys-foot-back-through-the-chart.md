---
id: the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart
kind: issue
title: The ruled and torus region doors read a query's foot back through the chart, in ulps that depend on its frame
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Filed from `a-sphere-doors-answer-moves-with-its-chart-frame` on closing it.

## What

`region::QueryFoot::read` (`crates/topo/src/boolean/region.rs`) sets a query's foot point by reading `eval(chart_point(p).uv)` on a cylinder, a cone and a torus, as it did on a sphere. The round trip lands a few ulps off the surface's own foot, and which ulps depends on the chart's frame. These doors read that point:
- the boundary pre-pass (`ruled_region.rs`, `torus_region.rs`, `on_boundary(foot.at(), band)`);
- the cone's apex and nappe readings (`ruled_region.rs`).

So a query placed at a band's edge can be decided by the frame's ulps, as the sphere door's was. These doors chart the record as stored, so no row moves with a frame today. A body turned onto another frame would.

## Wanted

Measure first: read a ruled and a torus face on several frames, at the band's zero and its escalation off an edge, as `sphere_region`'s `a_faces_answer_depends_on_no_chart_frame` does.
- If any frame answers otherwise, read the foot point in closed form without the chart, as the sphere's now is. That means the axis-orthogonal radial for a cylinder; the generator foot for a cone, outside the apex band; and the tube circle's radial for a torus.
- If none does, close this row with the measurement.
