---
id: step-promotion-reads-the-chart-sense-at-an-unchecked-midpoint
kind: issue
title: STEP surface promotion reads the NURBS chart's sense at the domain midpoint without asking whether the chart is regular there
status: open
opened: 2026-10-26
priority: P3
cost: E
---

Found by the class sweep of
`splice/the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex`
(a spline chart's normal `S_u × S_v` read at a point with no
regularity or crease reading).

`step-import`'s `recognize::chart_normal_mid` and
`recognize::chart_flipped` (`crates/step-import/src/recognize.rs`)
read the NURBS chart's `S_u × S_v` at the domain midpoint. This is the
orientation reference a promoted plane or cylinder is aligned to, so
the face's `same_sense` keeps its meaning. Neither asks whether the
chart is regular there:

- **An irregular midpoint.** Where `S_u × S_v` vanishes (coincident
  control rows, a degenerate parameterization),
  `chart_flipped`'s `nurbs_normal.dot(grad) < 0.0` reads `false` on a
  zero product and keeps the sense unflipped, whatever the chart's
  orientation elsewhere.
- **A midpoint on a knot run of multiplicity at least the degree.** A
  two-span degree-1 section puts one at the midpoint, where the jet
  reads one side's derivative. A promoted plane or cylinder is smooth,
  so the sign usually survives. But it rests on a parameterization
  that is only C⁰ there.

The readings to reuse are `geom_brep::regular_jet` and
`geom_brep::chart_normal_at`. Or read the sense at a sample the chart
is known regular at, and refuse the promotion where none is found.
