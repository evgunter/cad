---
id: a-closed-range-ending-on-a-knot-hulls-the-span-starting-there
kind: issue
title: span_range locates a range's END by span_of, so a hull over a range ending on a knot reads the neighbour span it only touches
status: open
opened: 2026-10-27
priority: P3
cost: M
---


## Found (QUAD, the knots-hugging area-gauge fix, 2026-10-27)

`KnotVector::span_range` (`crates/geom-core/src/spline/knots.rs`)
locates both ends with `span_of`, whose tie-break puts a knot value in
the span STARTING there. A range `[x, k]` that ends on knot `k`
therefore reports the span `[k, k']` as its last, though it only
touches it. For a `KnotVector` spline that span is never needed: the
clamped invariant (degree ≥ 1, interior multiplicity ≤ degree) makes
the spline continuous, so its value at `k` is the left span's endpoint
value, already hulled by the left span's coefficients.

The cost is width, not soundness, and it is unbounded when the
neighbour is a sliver: QUAD's `props::quad::net::range_hull` read a
1e9 derivative off a 1e-9 span for every area cell ending on that
knot and certified an area 2e7 wide (the `knots-hugging` row, closed
by the PR that filed this one, which fixed `range_hull` locally).

Callers that hull through `span_range` the same way, not touched:

- `crates/geom-core/src/interval.rs` `locate_spans` — the
  `SpanSet` every interval-parameter spline hull reads.
- `crates/geom-brep/src/ssi/enclose.rs` `cells` — the SSI enclosure's
  cell range (ssi territory).

A root fix would stop `span_range`'s last span at the one ending on
`end` when `end` is a knot and the range has positive width. That
moves bits wherever a hull ends on a knot, so it wants its own
re-baseline. `pcurve_cache/projected.rs` reads `.0` only and is
unaffected.
