---
id: a-sphere-doors-answer-moves-with-its-chart-frame
kind: issue
title: The sphere region door's answer at a band's edge moves with the frame its sphere is charted on
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
---


Found by the delta review of PR 6624 (D2), with D1, D3 and D4 of the same review folded in.

## What

The sphere region door (`crates/topo/src/boolean/sphere_region.rs`) answered a query at a band's edge differently depending on the frame its sphere was charted on. The geometry has no frame, so the answer should not move with one.
- **Seen in `sweep`**: on the record's own frame, `rays_in_band_of_a_curved_edge_read_it_once` read a hemisphere ray `o = 1e-8` off its edge in band (`curve_carrier_gap`, margin `9.999999886e-9` against escalate `1e-8`). On the door's own `z`/`x` frame, it read the same ray past the band. PR 6624 therefore charted the door on `Charted::framed(z, x)`.
- **The reading that depended on the frame** is the query's foot.
  - `region::QueryFoot::read` set the foot's point to `surface.eval(chart_point(p).uv)`, a trig round trip through the chart. It lands up to 6.5e-16 off the sphere's own foot, `c + R·(p − c)/|p − c|`, and how far depends on the frame.
  - The boundary pre-pass reads that point against every arc, and the paths' parallel reads its direction from it.
  - A query placed at the band's zero or its escalation is therefore decided by the frame's ulps.

## Done

- `QueryFoot::read` reads a sphere's foot point along the radius, which involves no frame. The chart's point stands only for a query at the centre, where the radius names no direction (`checked_direction`). The foot's coordinates stay the chart's, since they are what the legs start from.
- The sphere door charts the record as stored (`Charted::record`). `Charted::framed` has no caller left and is removed (D4).
- Pin `a_faces_answer_depends_on_no_chart_frame`: the cap of `carved` is read on five frames, at 200 azimuths, at ±1e-9, ±1e-8 and ±2e-8 off its rim. Every frame answers as the first.
  - Without the fix it fails on frame 1, at azimuth 1e-4, 1e-9 off: one frame gives `curve_carrier_gap`, another `Ok(None)`.
  - The sweep row fails too, on the record's frame.
- D1: `conic_crossings` names each way in and how it holds the ray plane's premise. `point_in_loop` certifies it, the projected read holds it as its own premise, and `contfp`, through `carrier_loop_side`, takes it unchecked from its caller.
- D3: `Charted`'s doc is reflowed.

The ruled and torus doors still read the query's foot back through the chart: `the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart`.
