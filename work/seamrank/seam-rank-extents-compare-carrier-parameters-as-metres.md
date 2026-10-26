---
id: seam-rank-extents-compare-carrier-parameters-as-metres
kind: issue
title: The crossing ranks compare a curved carrier's parameter extents against a band in metres
status: closed
opened: 2026-10-19
priority: P3
cost: M
closed: 2026-10-26
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

## Parked (2026-10-26, tracker sweep)

The comparisons this row is about (`discriminate::extent_before`, read
by `emit_topo::rank_reads` for `rank_crossings` and `rank_along_track`)
exist only to rank, and FORK-ORIENT (fork-log row 197; Ev, 2026-10-26;
PR 5705) rules that nothing is ranked. The build that retires the
rankers is WIRE's `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`,
so this row waits on it: if that build deletes the rank reads, it
closes as superseded; if any metered extent comparison survives it,
the row opens again for that.

## Moot (WIRE orchestrator, 2026-10-26)

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). The code this row is about is deleted: ranks, `RankRule`, canonical's rank rewrite and the carrier-parameter rank readers. PR 5758's FULL review checked that no premise of this row remains in editor-core.
