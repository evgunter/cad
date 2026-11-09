---
id: the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart
kind: issue
title: The ruled and torus region doors read a query's foot back through the chart, in ulps that depend on its frame
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6676
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

## Measured, and done

**Method.**
- The same face, with its 3D edges fixed, is read on several chart frames:
  - cylinder and torus: six (the seam turned to azimuths 0, 0.3, 1.0, 2.5 and −2.0, and the axis flipped);
  - cone: five (its axis is part of what it is).
- Faces:
  - a cylinder window `0.4 < u < 2.9`, `0 < z < 1.5`;
  - a cone window over slant `1 < v < 2`;
  - a torus patch `0.4 < u < 2.9`, `−0.5 < v < 0.8`.
- Queries are read at 60 stations along each edge, at ±5e-10, ±1e-9, ±5e-9, ±1e-8 and ±2e-8 off it. Each answer is classed as `Ok(side)` or its refusal's predicate.

**The foot point**, `|eval(chart_point(p).uv) − closed-form foot|`:
- cylinder 7.1e-16
- cone 6.0e-16
- torus 1.8e-15

**Queries answering by frame**, out of 2400:

| door | chart round trip | closed-form foot |
|---|---|---|
| cylinder | 536 | 0 |
| cone | 483 | 135 |
| torus | 712 | 133 |

- Every disagreement is at the band's zero (±1e-9) or its escalation (±1e-8). Offsets of 5e-10, 5e-9 and 2e-8 never disagree.
- The cone's and torus's remaining disagreements are all at ±1e-8, and all come from the legs: `bool_ruled_region_at`, `bool_torus_region_component`, `plane_crossing_root_in_span` and `bool_torus_region_at`. They are filed as `a-cone-or-torus-leg-carries-its-charts-frame-into-a-crossing-at-the-band`.
- Lattice and near-corner queries (3e-8 to 1e-4 from a corner) disagree nowhere: 568 queries on the cylinder and torus, 664 on the cone.

**Done.**
- `geom::Surface::foot` reads a point's foot in closed form, beside `chart_point`, from the surface's geometric fields alone. It never reads `u_ref` or a chart frame, and returns `None` where the direction is the zero vector, and for a spindle sheet or a spline.
- `QueryFoot::read` reads it for every door. This retires the sphere's arm in `region`.
- Pins:
  - `a_ruled_faces_answer_about_the_bands_zero_depends_on_no_chart_frame`;
  - `a_torus_faces_answer_about_the_bands_zero_depends_on_no_chart_frame`;
  - geom's `the_foot_is_the_charts_foot_on_no_chart` and `a_point_every_point_of_a_circle_is_nearest_has_no_foot`.
