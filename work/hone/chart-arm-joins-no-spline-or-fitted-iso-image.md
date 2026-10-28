---
id: chart-arm-joins-no-spline-or-fitted-iso-image
kind: issue
title: The curved join's Chart arm reads iso families off the analytic mint only: two pieces of one iso line on a spline chart, or of a fitted image, are left unjoined
status: open
opened: 2026-10-07
priority: P2
cost: H
---

## The finding

The curved join's `Chart` arm (`topo::boolean::edge_join`'s `locus_of`)
reads an edge's iso family through `geom_brep::chart_iso_family`, which
answers off `chart_pcurve`'s analytic derivation: `None` for a spline
chart (its `IsoLine` / `IsoArc` rows), for a fitted image, and for an
oblique section. Two pieces of one iso line on a spline chart are so
left unjoined, and `singular_at` reads no singular set for a spline
surface either. No `ci` row reaches it: the probe over every boolean
output found no valence-2 vertex between two `Chart` edges on a spline
chart.

## What it needs

The iso family of an `IsoLine` / `IsoArc` row read off its variant
(the structure the mint chose), and a regularity reading at a spline
chart's point (its partials' cross product, or the chart's degenerate
boundaries).

## Parked (2026-10-13, FUSE orchestrator, on a lane probe)

Nothing reaches the arm's `None` today. A temporary log in `locus_of` recorded every Chart reading:

- **Probe 1, `ci` suites.** Across topo, sweep and editor-core `ci` (8,462 tests), 258,976 readings came back. All were analytic `Harmonic` images and all were classified. No Nurbs or torus chart appeared, and no `Fitted`, `General`, `IsoLine` or `IsoArc` pcurve.
- **Probe 2, tilted sections.** Cylinder, cone, ball and donut against 48 tilted bricks: 716 bodies built and 1,468 readings, all classified.
- **Probe 3, two curved operands.** Seven pairs: 0 of 42 built (`GermFrameUnsupported`, `CurvedPierceUnsupported`, `CurvedBooleanUnsupported`, `FallbackExtentUnsupported`).
- **Probe 4, oblique cut.** The same-surface glue across an oblique cut built 0 of 144 (`GermEdgeCarrierUnsupported`).

Why nothing reaches it:

- **Fitted image:** it exists only on a spline chart.
- **Spline chart:** it reaches no join candidate, the same as the spline join.
- **Oblique section:** it lies on no iso line, so `None` is geometrically right for it. Joining two pieces of one oblique section needs carrier identity, not an iso family. Its only route is cleave's germ roster.

This row therefore waits on the first lanes that could mint such an edge: cleave's germ roster for the oblique case, and REACHHOLD's NURBS crossing rung for the spline case.

**For when it un-parks:** the row's "read the family off the minted `IsoLine`/`IsoArc` variant" covers only the spline-chart half. Even there, reading u-versus-v off `IsoLine`'s `pl` is a zero test on `T` unless the mint records the family beside the variant. The oblique half wants carrier identity instead. That is a design question to weigh with designers then.

## Probe 4 since the germ roster widened (SPLITDOOR, 2026-10-28)

PR 5940 (`splitdoor/tilted-plug-rosters`) reads two germ edges that are
sections of one plane and one cylinder or cone, each edge's two faces
door-verified one carrier with the other's, as one segment
(`sectors::same_construction`). Its review's oblique-halves probe (a
rod cut by `z = x` and its two halves glued back,
`crossing_equal_rods.rs` `a_rod_cut_obliquely_and_glued_back_is_the_rod`)
now builds the union, the rod at its closed form, at spins 0, 0.4 and 1,
in both orders, at ε = 1e-9, 1e-6 and 1e-12. The halves' intersection
and differences refuse `FallbackExtentUnsupported`, upstream of the
join. Measured on the review's branch; this row's status is unchanged.
Un-parked by SPLITDOOR at PR 5940's merge (2026-10-28): its trigger closed, the oblique case now
reaches this lane, and no REACHHOLD row for the NURBS crossing rung exists to re-park the spline
half on. The design question above (family recorded at the mint, carrier identity for the oblique
half) stands for whoever takes it.
