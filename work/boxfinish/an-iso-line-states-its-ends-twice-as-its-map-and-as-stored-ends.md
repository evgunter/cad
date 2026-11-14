---
id: an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends
kind: issue
title: An IsoLine states where its ends are twice, as its map and as stored ends, kept in step by hand and certified by different checks
status: open
opened: 2026-11-14
priority: P1
cost: M
design: true
refs: [6959]
---


Raised by the review of PR 6959 (the chart-ends unit,
`check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends`),
2026-11-14, as its main style finding. That PR lands with the hole
below closed inside the representation it built; this row is the
representation.

**What is there.** `Pcurve::IsoLine` holds a map `p0 + pl·t` over its
edge's parameter and, since PR 6959, `ends: Option<[Point2; 2]>`, the
chart points at the two ends of the span it is stored on. The PR
measured why an end has to be stored: an edge's own `t1` is a computed
length, an enclosure a few ulps wide at `Interval` and box-wide over a
parameter box, and no arithmetic re-reading of `p0 + pl·t1` recovers an
exact chart 0 or 1 from it.

**What is wrong with it.**
- The two say one thing and are kept in step by hand at about ten sites
  (`split_at`, `with_end_at`, `over_span`, the re-anchor, `map_chart`,
  `shift_chart_v`, the lift). One stale end was found by two rows
  breaking.
- They are certified by different checks: the ends by check 3's end
  stations, the map by check 4's envelope. The review demonstrated the
  drift: an image whose stated end sat ε/2 off its map certified with
  `envelope = 0`, and the rectangle certificate's pad is that envelope.
- `Option` conflates four states: placed by the mint; re-read through
  the map after a span changed; selected in band by the importer; none.
  `None` cannot tell "nothing to state" from "forgot".
- Both are functions of the same two placed points and the span.

**The question.** What does a chart image that is a straight row or
column of its chart store, so that an end its construction placed on
the chart's edge reads as exact at every scalar, and nothing is stated
twice? Whoever weighs it should look at what the quad lane's rectangle
certificate actually needs (that the trim IS the chart's rectangle,
which may be a discrete fact and not two points), and at
`docs/DESIGN.md` `[edges]`: for a `Chart` edge the pcurve is the
description, not a cache.

**Waits on it:** `an-iso-arc-rims-end-reads-through-its-map-at-interval`
(filed by PR 6959), so that `IsoArc` does not get a second copy of the
same shape.
