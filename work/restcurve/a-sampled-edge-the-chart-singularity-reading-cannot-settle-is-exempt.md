---
id: a-sampled-edge-the-chart-singularity-reading-cannot-settle-is-exempt
kind: issue
title: A sampled edge the chart-singularity reading cannot settle on or off is exempt from tier 3
status: open
opened: 2026-10-30
priority: P3
cost: M
---


## What

`geom_brep::chart_singularity::locate` reads a sphere pole's or cone
apex's distance to an edge on a sampled carrier (anything but a line or
a circle) by refining the edge's samples against the carrier's speed
bound (`chart_singularity.rs:532`, `sampled_near`). Where that settles
neither on nor off, it answers `Unread::Carrier`, and tier 3's
chart-singularity arm exempts the face (`crates/topo/src/validate.rs:8223`;
the not-yet-checked list names it). Two causes remain after PR 6054:

- **the budget**: `SAMPLED_BUDGET` bisections (`chart_singularity.rs:512`)
  run out before the band-wide stretch at the point resolves, which
  takes a speed bound far above the carrier's speed there (the slow end
  of a long ellipse; a spline whose control net is much wider than the
  curve). Row: `geom-brep/tests/chart_singularity_locate.rs`
  `a_loop_winding_twice_and_an_unsettled_carrier_are_exempt`.
- **no bound**: a carrier whose speed bound is not a finite positive
  number (`geom::carrier_speed::speed_range`'s sup).

Import refuses the same reading (`foreign_carrier`), so nothing passes
import that tier 3 would refuse; the exemption is a body at rest built
otherwise.

## Owed

A distance reading that settles a sampled carrier without the speed
bound's slack: a local speed bound per span (the spline's own
derivative hull, a conic's speed at its span), so the refinement's
count follows the curve rather than its worst stretch; or a closed-form
foot for the ellipse. Then the exemption is the in-band reading alone,
which is `Unread::Boundary`.

Filed by the RESTCURVE implementer in PR 6054's fix pass (adjudication
A2, Q6).
