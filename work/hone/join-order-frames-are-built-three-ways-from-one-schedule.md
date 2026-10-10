---
id: join-order-frames-are-built-three-ways-from-one-schedule
kind: issue
title: The in-plane frame from SCHEDULE is built in containment's ray parity and splitting::order, and walked a third way in solid_contain
status: open
opened: 2026-10-10
priority: P3
cost: E
---


Found by PR 4224's review (CLEAVE, the join order's frame), not merged
there: one rule per home.

The rule "project a `SCHEDULE` member into a plane of unit normal `n`,
gate its in-plane fraction levered by a length, normalize it to `u`,
and take `v = n × u`" is written twice and read a third way:

- `splitting::containment`'s in-plane ray parity (the closure handed
  to `ray_walk::walk` in the loop walk: `d_raw = r − n(n·r)`,
  `Margin::levered(|d_raw|/|r|, extent)` under the caller's arm row,
  then `d_raw.normalize()` and `normal.cross(d)`);
- `splitting::order::in_plane_frame` (the join order's frame: the same
  projection, gate (`split_join_frame_arm`), normalization and cross,
  over the axis or the oblique half of the table);
- `boolean::solid_contain`'s closest-hit sweep walks the same table as
  raw space directions (`r.map(T::from_f64).normalize()`), and
  `boolean::sphere_region` maps the table again.

The two projecting copies differ only in their arm row, their refusal
and which members they walk (the loop walk takes the whole table in
order through `ray_walk::walk`; the order takes a fixed half and stops
at the first definite member). A change to the gate's lever — the
class the rim-dimensional audit fixed once at each site — has to be
made at both.
