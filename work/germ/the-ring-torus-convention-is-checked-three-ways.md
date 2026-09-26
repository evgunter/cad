---
id: the-ring-torus-convention-is-checked-three-ways
kind: issue
title: The ring-torus convention (R > r, rho > 0) is checked in three spellings with three error types
status: open
opened: 2026-09-26
priority: P3
cost: D
---

## What

Three doors each check that a torus is a ring torus, each in its own
words:

- `crates/topo/src/chart_region.rs`: `gate(major_radius - minor_radius, "torus")`,
  into `ChartRegionError`.
- `crates/topo/src/boolean/solid_contain.rs` `point_on_torus_in_face`:
  `bool_torus_frame_radius`, a banded check that the point's `ρ > 0`,
  escalating `PointInSolidError`.
- `crates/topo/src/face_normal.rs` `face_outward_normal_at`:
  `bool_pierce_normal_ring_torus`, a banded `R − r > 0`, answering no
  arm (added by PR #3265).

They check two different data (`R − r` of the carrier, and `ρ` at a
point), raise three error types, and use three predicate names. A
change to the convention, or to its band, is a change to all three.

## The fix's shape

One `geom_brep` predicate for the carrier's premise (`R − r` decided on
the band), read by all three doors and each mapping it into its own
error type. The point-level `ρ > 0` check follows from it for a point
certified onto the surface, so it can be removed or stated as that
consequence.

## Home

GERM (filed by PR #3265's final pass). `chart_region.rs` is CHART's
ground, so the unification crosses that seam, which is why it was not
done in that PR.
