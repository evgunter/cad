---
id: the-euler-splits-interiority-gate-meters-an-end-gap-at-the-speed-floor
kind: issue
title: split_edge's interiority gate meters the split point's gap from each end at the speed floor, so it refuses a crossing past the band of a thin ellipse's fast end
status: closed
opened: 2026-11-09
priority: P1
cost: E
closed: 2026-11-10
branch: onecopy/edge-ends-turns
pr: 6710
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
the e2e pin `a_crossing_at_a_thin_ellipses_fast_end_refuses_at_the_split` (now `…_splits`)
to a split.

## Closed by `onecopy/edge-ends-turns`

Measured on the e2e pin, a 1000 × 1 ellipse's fast ends cut 50, 500 and
5000 zeros in:
- with the split gate routed alone, every cut passed the gate and the
  child from the end to the root refused at check 2:
  `IntervalNotForward { Zero, margin 5e-11 }`, which is `Δt·inf` for a
  child whose chord is 5e-8;
- with check 2 reading the chord too, every cut splits both crossings,
  at ε 1e-9, 1e-12 and 1e-6
  (`a_crossing_at_a_thin_ellipses_fast_end_splits`).

The fixes:
- `split.rs`'s interiority is `Span::place_param_read`.
- Check 2's periodic arm is
  `geom_core::period::Extent::forward_by_arc`: forward at the floor, or
  by the chord between the span's ends (`interval_span_chord`) with its
  sense at the sup speed (`interval_span_sense`). It is pinned in
  geom-core by `forward_by_arc_reads_the_chord_where_the_floor_under_reads`.

Provenance of check 2's floor reading, looked for before changing it:
`git log --all -S` on an unshallowed clone finds check 2 born in M2 PR 3
(`e1600790f93`, 2026-07-19), a fix pass, and its floor wording in
`certify.rs`'s code docs (`e1d18d25b86`, the turn rule's move). No
DESIGN.md clause and no README design page states how check 2 meters its
span. `[tolerance.margins]` clause (iii) requires a parameter value to
cross to model space through a metric door; the chord is a model-space
norm and crosses nothing.
