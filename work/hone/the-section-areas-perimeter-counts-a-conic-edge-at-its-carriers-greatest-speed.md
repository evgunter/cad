---
id: the-section-areas-perimeter-counts-a-conic-edge-at-its-carriers-greatest-speed
kind: issue
title: split_section_area counts a conic section edge at its carrier's greatest speed, so a section whose mean width is past 10 eps can read inside the gap and refuse
status: open
opened: 2026-11-20
priority: P3
cost: C
---

Found in the review of PR 7146 (`oneround/section-area`), 2026-11-20.

## What

`Sweep::certify_section_area` (`crates/topo/src/splitting/join.rs`)
decides `2·|A| / P` under `split_section_area`. For a circle or ellipse
edge it adds `conic.speed_hi() · |Δt|` to `P`: the carrier's larger
semi-axis times the span, an upper bound on the arc's length that is
exact only where the edge sits on the carrier's fast stretch. Its doc
states the choice: a larger perimeter only shrinks the margin, so it
"refuses more, never less".

`geom::carrier_speed::length_hi` bounds the same length over the edge's
own stretch, and `geom_brep::props::closed_segment_term` already returns
it beside the segment term this site reads.

## Measured

The plate of
`crates/sweep/tests/a_corner_cut_on_a_far_wall_reads_its_sections_width.rs`
(one side an arc of radius 1e4), split by the plane of normal
`(0.05, 1, 1)` (normalised) `12 ε` inside its corner, at ε = 1e-9, PR
7146's head:

- the section's geometric mean width, from its three corners: 1.159e-8,
  past `10 ε`;
- the margin the predicate reads: 9.6648e-9, inside `(ε, 10 ε)`, so the
  split refuses, `Join(Escalated { predicate: "split_section_area" })`.

The section's arc side is on an ellipse of semi-axes (14151, 10000),
counted at 14151 per unit of parameter.

Before PR 7146 this split returned `Ok`, on a reading of the area four
orders off; the refusal is what the predicate as defined says.

## The question

Whether the over-statement is a choice to keep or a refusal to remove.
Reading `length_hi` in its place makes the margin the section's own to
within a piece of the carrier's speed. It moves verdicts only toward
`Positive`, on sections whose width is within the axis ratio of `10 ε`.

