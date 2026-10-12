---
id: a-line-edges-span-bounded-by-a-conic-crossing-is-not-recorded
kind: issue
title: A line edge's span in a face bounded by a crossing of the face's conic boundary edge is not recorded
status: open
opened: 2026-10-11
priority: P4
cost: M
refs: [an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap]
---


Unmeasured, read. The blind spot of `an-apex-edge-…`'s sweep.

## What

`boolean::ops::record_spans` takes a bound of a line edge from an
edge-edge record only where the other edge is a line too: it computes
the crossing as the nearest point of two lines. Where a line edge lying
in a plane face leaves it across one of the face's arc boundary edges
(a shelf edge across a half-disc cap's rim), that crossing bounds no
span, so the edge's record against the face is not minted, whatever
records the op made at the crossing.

## Why it is P4

The census refuses every such configuration already: pass 4 cuts the
overlap at the conic crossing (`CutAt::ConicCrossing`) and backs that
bound with nothing, and CONTACTHOLD measured the same seat refusing
(`contacthold/census-edge-pass-reads-no-line-conic-crossing`, closed;
CHART's `a-planar-face-with-a-conic-trim-has-no-region-door` is the
census side). So the missing record changes no verdict today.

## Owed

First measure whether the op records the crossing at all (an
edge-edge record of the line and the arc). If it does, the bound is
the line's point on the arc's carrier, read where the op decided it;
then the span is minted as the line-line case is.
