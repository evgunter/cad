---
id: the-euler-splits-interiority-gate-meters-an-end-gap-at-the-speed-floor
kind: issue
title: split_edge's interiority gate meters the split point's gap from each end at the speed floor, so it refuses a crossing past the band of a thin ellipse's fast end
status: open
opened: 2026-11-09
priority: P1
cost: E
---


Found by PR 6687's FULL review (MINOR-1), beside
`a-plane-crossing-past-the-band-of-a-thin-ellipses-fast-end-read-at-the-end`.

## What

`crates/topo/src/split.rs` (`split_edge`, the `split_edge_param_interior`
gate) decides `(t − t₀)·floor` and `(t₁ − t)·floor`, `floor` the
carrier's speed floor (`geom::carrier_speed::speed_floor`: an ellipse's
smaller semi-axis). The plane door now places a root past the band of a
thin ellipse's fast end by its chord. Within `Kε … (sup/inf)·Kε` of that
end, the gate reads it at the end or in band and refuses
(`SplitParamNotInterior` / `SplitParamEscalated`), so `split_reduce`
raises `CrossingInsertion`. It is sound (a Zero refuses, never
splits). It is a liveness hole: the two readings of one root disagree.

**Same shape, one step on: certification's check 2**
(`geom-brep/src/certify.rs` `decide_turn`, `interval_span_forward`).
It decides a span forward at the same floor (`Extent::forward` at the
carrier's inf rate), so a child piece of arc `ℓ` at a fast end reads
`ℓ·inf/sup` and refuses `IntervalNotForward` below `(sup/inf)·Kε`. That
too is sound: a forward claim needs a lower bound on the arc. The
split's child is in that window wherever the root is, so routing the
split gate alone moves the refusal from `split_edge` to the child's
certification. The span's length is the question there, and arc length
has no sup-side reading that certifies it; the chord between the
child's ends is a lower bound on its arc that no speed ratio shrinks.

## Owed

Route the split gate to `curve_pair::Span::place_param` (the span's one
reading), then read check 2 at the chord between the span's end points
as well as `Δt·inf` (forward where either is past the band), and flip
the e2e pin `a_crossing_at_a_thin_ellipses_fast_end_refuses_at_the_split`
to a split.
