---
id: seam-rank-extents-compare-carrier-parameters-as-metres
kind: issue
title: The crossing ranks compare a curved carrier's parameter extents against a band in metres
status: open
opened: 2026-10-19
priority: P3
cost: M
---

## What

Found by the fold lane of PR 5347 (its sweep of the S1 fix). The
ranking comparisons in `crates/editor-core/src/names/emit_topo.rs`
(`extent_before`, through `rank_crossings` and `rank_along_track`'s
shared tail `rank_reads`) compare positions along a carrier by its
parameter. On a curved carrier the parameter is in radians (circle,
ellipse) or a spline's knot units. The margin they are judged against
is in metres. The verdict does not depend on which piece reads, since
every point is placed on one carrier. But the band is not the
documented one: on a circle of radius r it is r times too tight or too
loose, and on an ellipse its width varies with the speed of the curve.

PR 5347 metered `Track::sign` by the carrier's greatest speed
(`Margin::metered_sup`). This is the same fix, applied to the extents.

## Next

Meter the extent comparisons the same way, or compare arc length.
Then pin it on a large and a small circle: crossings that just separate
at the band should rank, and crossings that just fail to should
escalate, alike at both radii.
