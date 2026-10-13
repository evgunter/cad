---
id: an-imported-off-iso-line-on-a-bspline-face-takes-no-general-row
kind: issue
title: an imported off-iso line on a B-spline face has no rung that reaches its General image
status: open
opened: 2026-10-12
priority: P2
cost: M
---


Asked of `pcert/fitted-row-reads-carrier-and-chart` by its
orchestrator, and measured there by reading the ladder plus a
topo-level probe. Not reproduced through a STEP file.

## What happens

An imported edge whose carrier is a `Line` (promoted by D7, or written
as one) and that runs off-iso across a B-spline face meets the
adoption ladder (`crates/step-import/src/adopt.rs`, `adopt_edges`):

- **IsoCurve rung**: `iso_curve_candidates` offers a promoted line only
  the wall's boundary COLUMNS (`line_column_match`), so an off-iso line
  gets no candidate;
- **Intersection rung**: refused at the edge certificate, which demands
  a spline carrier for a plane × NURBS or NURBS × NURBS pair
  (`work/exch/d7-promotes-an-off-iso-straight-spline-the-intersection-rung-then-refuses.md`);
- **conventional rung**: for a plane × wall pair (`nurbs_rim`) the line
  adopts as an image in the plane's chart. On the wall, the pcurve mint
  reaches its `Chart`/`Scaffold` line-rim arm, which picks a row through
  the start foot and mints a wrong `IsoLine`. The import then refuses
  the face (`LoopDiscontinuity` on the probe,
  `work/pcert/an-off-iso-straight-edge-on-a-spline-face-refuses-its-rows.md`).
  For a wall × wall pair no rung is left.

So no rung reaches the `General` image the fitted-grade row can now
certify for a line.

## What remains

Once the pcert row is fixed, the plane × wall case adopts through the
conventional rung and mints `General`. The wall × wall case also needs
the edge certificate to take a line carrier, or recognition to leave
the spline alone (the D7 row).
