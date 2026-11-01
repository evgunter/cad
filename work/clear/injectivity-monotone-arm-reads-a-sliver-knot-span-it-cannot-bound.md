---
id: injectivity-monotone-arm-reads-a-sliver-knot-span-it-cannot-bound
kind: issue
title: A NURBS wall whose interior knot sits a few ulps off a bisection point never certifies against itself: the cell beside the knot restricts to a sliver span, rect_jet's degree/length scaling blows its Jacobian hull up, and the monotone arm splits that iso line until the pair budget runs out
status: dispatched
opened: 2026-10-31
priority: P0
cost: M
---


## What was found (CARVE's embedding-certificate measurement, 2026-10-31)

The body-level injectivity sweep (`Sweep::run`, `StrictlyPositive`,
one body against itself) on the planar arc-spine loft of
`crates/sweep/tests/bool6_per_slab_stacking.rs` (a 0.1 square, unit
radius, 17 stations, v-degree 3) refuses `Budget(Pairs { 65536 })`
after about 25 s at 3.5 rad, and `Pairs { 262144 }` after about 96 s.
With the curl, it flips between that and `Holds` in about 0.02 s per
wall:

| curl (rad) | 3.0 | 3.1 | 3.2 | 3.3 | 3.4 | 3.45 | 3.5 | 3.6 | 4.0 | 5.0 | 6.0 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| each wall against itself | holds | holds | budget | holds | budget | holds | budget | budget | holds | holds | budget |
| interior v knots 1–3 ulps off a dyadic `m/2^d` | none | none | 2 | none | 7 | none | 7 | 8 | none | none | 10 |

The correlation is exact over these eleven curls. The stations are
equally spaced, so every knot is `k/16` in exact arithmetic, and the
chord-length sums round it 1–3 ulps either way, or not at all. The
same holds on the tour: `skinned`'s `s_duct` has one knot at `0.5 + 1
ulp`, and every wall against itself refuses on the budget (deepest 22),
while each of its cross pairs holds in about 0.15 s.

Per pair, the stuck cells are each wall against ITSELF; caps and
cross pairs hold. The frontier grows like a line locus (depth 20 at
16384 pairs, about 5700 pairs unstarted), not like a point or an area.

## The likely mechanism (read, not instrumented)

Bisection lands exactly on `m/2^d`. With the knot an ulp or two past
it, the cell on one side holds a span of the knot vector a few ulps
wide, and so does every descendant cell with an end on that line.
`NurbsSurface::rect_jet` (`crates/geom/src/surfaces/nurbs.rs`) hulls
each span's `restricted_jet`. That scales the restricted net's
differences by `degree / span length`. Over a span ulps wide the
differences are rounding, so the hull is huge and never shrinks. The
monotone arm (`injective::monotone`) reads `rect_jet` over the hull of
two meeting cells, cannot decide that `least` is positive, and splits.
That happens again at every depth along the line.

## What a fix needs

Either `rect_jet` drops, or bounds by its neighbour, a span narrower
than the cell's rounding (the jet is continuous across a knot of
multiplicity at most the degree), or the split snaps a bisection point
to a knot within a few ulps of it. To confirm, run a `rect_jet` probe
on the 3.5 rad wall at the cell `v in [0.375, 0.4375]`.
`carve_embed_scratch_arc_pairs` with `CARVE_EMBED_CURL` (in
`crates/sweep/tests/carve_embed_scratch.rs` on
`carve/embedding-certificate`) prints the knots and runs each pair.
