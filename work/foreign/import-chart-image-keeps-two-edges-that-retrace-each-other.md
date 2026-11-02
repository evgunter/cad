---
id: import-chart-image-keeps-two-edges-that-retrace-each-other
kind: issue
title: step-import's chart image takes out only one EDGE_CURVE walked back, not two distinct EDGE_CURVEs that retrace each other
status: open
opened: 2026-10-30
priority: P4
---


`step-import`'s pole-and-apex normalization reads a face's loops' sides
on their chart images (`crates/step-import/src/normalize/singularity.rs`,
`image_edges`). Since RESTCURVE's PR 6088 the image is structural: a use
followed by a use of the SAME `EDGE_CURVE` the other way is a spur and
comes out (`geom_brep::chart_image::without_reversals`).

Before it, `geom_brep::chart_singularity`'s `without_spurs` / `retraces`
took a spur out geometrically: two consecutive edges with swapped vertex
tags, the same stored interval and the same point at its middle within
the rounding charge. That also took out two DISTINCT `EDGE_CURVE`s that
retrace each other (a file that states a slit as two coincident edges).
The structural reading keeps them, so their tip's turn of half a turn
reaches the side reading (`sphere_pole_left`, `walk_loop`), which may
read such a loop undecided where it read it before.

No corpus file states such a pair (the review of PR 6088 found none). In
a body the pair is two distinct edges on one segment, tier 3's coincident
edges, which `topo::sector_shape::SlitTips` reads; at import the question
is whether such a pair is adopted at all, or refused or merged before the
normalization reads it. P4: not observed.

Signed: (RESTCURVE implementer)
