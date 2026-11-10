---
id: an-in-span-read-meters-a-raw-end-margin-named-anywhere
kind: issue
title: Two in-span readings meter a parameter's raw margin from a span's ends, unread on its copy a turn away; is each parameter named about the span's middle at Interval?
status: closed
opened: 2026-11-10
priority: P3
cost: E
refs: [a-turn-renamed-parameter-is-placed-against-a-span-at-four-sites]
closed: 2026-11-10
---


Filed from PR 6727's review (F5). That PR gave the copy rule one home, `geom_core::period::Period::span_margins`: each end's margin read on the parameter's own copy and on its copy a period away, against the other end. `curve_pair::Span` and ring_path's span readings read it.

Two more readings decide a parameter against a span's ends on its raw margins, `t − t0` and `t1 − t`:
- `crates/geom-brep/src/chart_singularity.rs` (~`:600`, `strictly_inside`, row `chart_singularity_in_span`): an edge parameter strictly inside the edge's interval.
- `crates/topo/src/boolean/reduce.rs` (~`:5034`, row `bool_wall_root_in_span`): a wall root against its span's ends.

The review also named `crates/topo/src/split.rs`'s interiority gate. It now reads `Span::place_param_read` (PR 6710), so it has the copy rule already.

**The question, per site.** At `Interval`, is the parameter named about the span's middle, so that a box across the middle's antipode can be named whole on one copy? If so, does the span reach a near-full turn, so that the box's other copy can reach into it? Then that site reads `Period::span_margins` (or `Span`). Otherwise the site should cite the reason its spans cannot reach the antipode.

Not fixed here: the review asked for the question only.

## Closed (2026-11-10, `onecopy/in-span-end-gap`)

Both sites are ported to `geom_brep::curve_pair::Span` (PR 6767's `end_gap` and the span's own `place_param`), and the question is answered per site.

- **`chart_singularity::nearest_on_edge`** (`chart_singularity_in_span`) is a span end gap. It reads `Span::end_gap(t, end, GapMeter::Clear)`, the gap metered at the least speed over it, under which a decided `Positive` is sound, still proved at the least band.
  - **No period, and why.** `foot` names the parameter at or past the edge's first end, a turn on at most (`Period::at_or_above`), not about the edge's middle. So the one place an enclosure can lie astride two copies is that end, a vertex `locate` reads before the edge. Off it, a box is named whole on the copy the edge runs from, and its other copy lies a full turn on, past any edge's last end. The site states this.
  - The carriers are a line and a circle, whose speed is constant, so the gap's metres are the old `m · speed_at(t)`'s up to the window bound's rounding allowance, and no sign moves.
- **`reduce.rs`'s `wall_crossing`** (`bool_wall_root_in_span`) reads each root against its span by `Span::place_param`: at an end by its chord, else inside, else past by its side of the ends, each also read on its copy a turn away (`Period::span_margins`).
  - The door names a root within half a turn of the span's middle, and a wall root's edge can be a full circle. So at `Interval` a root's enclosure astride the middle's antipode can have its other copy inside the span, which that copy's reading keeps.
  - It was a three-way reading on `gap · speed_at(end)`, the pointwise speed at the end, for an ellipse no bound on the arc over the gap. `bool_wall_root_in_span` retires, and with it `work/scalar/carrier-cross-in-span-meters-by-the-local-speed-against-the-rate-pair-rule.md`, whose last site it was.

**Measured: no decision moved.** A temporary probe read each site's old reading beside the new one at every call, over geom-brep, topo, sweep, editor-core and step-import's `ci` profile. It was measured once, on 2026-11-10, and no row guards the counts.

| | default ε | ε 1e-12 |
|---|---|---|
| chart_singularity, both gaps | 62955 | 61090 |
| wall roots | 27962 | 27806 |
| decisions that differ | 0 | 0 |

About 0.7% of the log's lines (633 and 896) were interleaved by concurrent writers and could not be read, so they are not counted.
