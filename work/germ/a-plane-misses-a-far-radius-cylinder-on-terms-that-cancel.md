---
id: a-plane-misses-a-far-radius-cylinder-on-terms-that-cancel
kind: issue
title: trace::plane_misses reads a parallel cylinder's miss as |(origin − point)·n| − radius, two far terms cancelling uncharged
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [7553]
---


Filed by GERM's `germ/harmonic-belly` lane (PR 7553), from the review's correction of its sweep table.

## Finding

`topo::boolean::trace::plane_misses` (`crates/topo/src/boolean/trace.rs`) decides `trace_plane_misses` on `off(anchor, reach)`, which is `|(anchor − point)·normal| − reach`. That is two plain terms:
- **The offset** `(anchor − point)·normal` is a plain dot, not `geom::plane_offset` (`difference_dot`). With the anchor stored far along the plane from `point`, it rounds at `u` of that distance.
- **The parallel cylinder's arm**, `off(origin, radius)`, cancels two terms of the radius's size when the radius is far and the plane grazes the wall. The offset of the cylinder's axis and its radius are then both about `far`, while their difference is of the band's order.

So the miss is read at `u·far`, uncharged: it can be definite on a gap within the band, or against its exact sign. This is the same class as the far-centred conic's belly that PR 7553 re-poses. PR 7553's sweep table first called this site "a stored radius against an offset, nothing of a conic's size cancels". That was wrong for a far radius: the radius is stored exactly, but the offset it is compared with is about `far` and rounded.

## Settles when

The miss is read to its own digits: the offset through `plane_offset`, and `|d| − r` as `(d² − r²)/(|d| + r)` with `d² − r²` formed exactly, or about a point of the wall nearest the plane. A row at ε 1e-12 must show it: a cylinder of radius 1e4 to 1e6, its axis parallel to a plane grazing its wall, gap within ±2× the escalation, judged exactly on the stored data, 0 wrong.
