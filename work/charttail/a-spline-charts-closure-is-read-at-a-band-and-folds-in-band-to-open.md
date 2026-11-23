---
id: a-spline-charts-closure-is-read-at-a-band-and-folds-in-band-to-open
kind: issue
title: A spline chart's closure (whether a channel is periodic) is read at a band and folds an in-band net to open; Ev leans exact for now
status: open
opened: 2026-11-21
priority: P3
cost: M
refs: [7287]
---


The separable half of the collapsed-column fork ([ev] PR 7287, 2026-11-21).

## What

PR 7287 made a spline chart's singular set exact. DESIGN's Extensibility paragraph now says that for `Nurbs` and `Approx` "the kind fixes the rule, not the points". The other half of a spline chart's shape, whether a channel closes, is still read at a band: `net_closes_as` folds an in-band net to "open". PCERT has an open row on that fold. One designer proposed the same exact rule for closure (first and last rows identical).

## Ev's answer (PR 7287, 2026-11-21)

> in the separable case, i don't know--it seems like it might need the band to accept everything it "should", in some sense, but also it seems not terrible if we require things to be exactly periodic to make our lives easier, and fix it later when necessary

So this is not a ratified rule. Requiring exact periodicity is acceptable as the working choice, and the band is to be revisited when a case needs it. Before building, read PCERT's fold row and decide whether the exact closure rule lands with it. If closure becomes exact, DESIGN's sentence gains the closure half in that PR. That PR changes ratified text, so it waits for Ev.
