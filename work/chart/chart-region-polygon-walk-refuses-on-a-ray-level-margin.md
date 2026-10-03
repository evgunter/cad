---
id: chart-region-polygon-walk-refuses-on-a-ray-level-margin
kind: issue
title: chart_region::point_in_polygon refuses on a ray's in-band side or advance margin where the 3-D loop walks abandon the ray
status: open
opened: 2026-10-02
priority: P3
cost: E
---

Unmeasured: no chart fixture is known to reach it. Found by the sweep of
CLEAVE's far-plane fix
(`work/cleave/point-in-solid-reads-in-band-against-a-face-plane-far-from-the-face.md`).

`chart_region::point_in_polygon` (`crates/topo/src/chart_region.rs`)
runs `ray_parity::on_boundary` first, then each `SCHEDULE_2D` member's
`ray_parity::ray_verdict(...).map_err(escalate)?`. An in-band
`chart_region_side` or `chart_region_advance` therefore refuses the
whole query. Those are a vertex in band of the ray line, or a crossing
in band of `q`.

Both 3-D walks now treat such a margin as being about the ray, not
about `q`:

- `splitting::containment::carrier_walk`, the arc walk, has done so
  for a while ("WHY A RAY-LEVEL MARGIN RETRIES").
- `polygon_walk` does so since the far-plane fix.

Once the boundary pre-pass places `q` off every segment by more than
the band, such a margin abandons the ray. The first one is the refusal
only if no ray decides.

The repro shape: a chart polygon with a vertex far ahead of `q`, in
band of `q`'s first `SCHEDULE_2D` line (an axis-aligned cap is the
common case). That query refuses where the next member would decide.

`chart_bound::parity` maps the same `Err` to `None`, which is
conservative there, and needs no change.
