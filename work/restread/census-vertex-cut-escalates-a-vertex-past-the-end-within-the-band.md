---
id: census-vertex-cut-escalates-a-vertex-past-the-end-within-the-band
kind: issue
title: The edge-face vertex cut escalates a boundary vertex lying past the edge's end by less than the band where its distance to that end is undecided, though a vertex there can cut nothing
status: open
opened: 2026-10-18
priority: P3
cost: E
---

## What

Found by review 2 of PR 5261 (its NOTE 8). `crates/topo/src/census.rs`
`ef_overlap_cells` cuts an edge at a face's boundary vertex through
`on_edge_interior_reading`, which is `geom_brep::curve_pair::Span::contact`.
`contact` reads the span first, through `place_at`. For a foot past the
edge's end, the chord from the foot to that end is decided
(`curve_pair_span_end_chord`, then `_side` / `_straddle`). Where the
vertex lies past the end by less than the band, and that chord's
reading is undecided, the escalation is pushed.

But a vertex there can cut nothing. Within the band of the end, it is
the end's own cut, and past it, it is off the segment. So the census
refuses a configuration that has one answer: a sub-band proxy, the
shape this PR removed at the band's scale.

## The shape to give

Read the place's escalation through `census::read_undecided`. Under
both readings, end and past, the cut is "none". If the vertex's gap off
the line is not `Zero` either, nothing else is at stake. The reading
belongs to `contact`'s callers, of which pass 2's (`pair_vertex_edge`)
has the same question. Measure first: the probe's default grid showed
no `curve_carrier_*` / `curve_pair_span_*` escalation from pass 4 at
ε = 1e-9, 1e-12 or 1e-6 after PR 5261, so this needs a constructed
witness.
