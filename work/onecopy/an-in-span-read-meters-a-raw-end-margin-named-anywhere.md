---
id: an-in-span-read-meters-a-raw-end-margin-named-anywhere
kind: issue
title: Two in-span readings meter a parameter's raw margin from a span's ends, unread on its copy a turn away; is each parameter named about the span's middle at Interval?
status: open
opened: 2026-11-10
priority: P3
cost: E
refs: [a-turn-renamed-parameter-is-placed-against-a-span-at-four-sites]
---


Filed from PR 6727's review (F5). That PR gave the copy rule one home, `geom_core::period::Period::span_margins`: each end's margin read on the parameter's own copy and on its copy a period away, against the other end. `curve_pair::Span` and ring_path's span readings read it.

Two more readings decide a parameter against a span's ends on its raw margins, `t − t0` and `t1 − t`:
- `crates/geom-brep/src/chart_singularity.rs` (~`:600`, `strictly_inside`, row `chart_singularity_in_span`): an edge parameter strictly inside the edge's interval.
- `crates/topo/src/boolean/reduce.rs` (~`:5034`, row `bool_wall_root_in_span`): a wall root against its span's ends.

The review also named `crates/topo/src/split.rs`'s interiority gate. It now reads `Span::place_param_read` (PR 6710), so it has the copy rule already.

**The question, per site.** At `Interval`, is the parameter named about the span's middle, so that a box across the middle's antipode can be named whole on one copy? If so, does the span reach a near-full turn, so that the box's other copy can reach into it? Then that site reads `Period::span_margins` (or `Span`). Otherwise the site should cite the reason its spans cannot reach the antipode.

Not fixed here: the review asked for the question only.
