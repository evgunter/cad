---
id: the-joins-in-band-class-reading-ends-two-ways
kind: issue
title: The join's in-band class reading ends lever-only at the boolean and sized on the pole lever at the join door
status: open
opened: 2026-11-04
priority: P3
cost: E
---


## What

One decision, the join's class reading in band (a cylinder ruling turned
3ε, pcert E3's `class_zero_rows` fixture), ends two ways at a build:

- the boolean (`crates/topo/src/boolean/edge_join.rs`,
  `JoinUndecided::write`, the `ChartClass(Undecided)` arm,
  `diag.lever_ending(COINCIDENCE_RECOURSE)`): "whether an edge meeting
  another at a vertex on one curve runs along a cylinder's ruling or
  around it is undecided: … Recourse: move the geometry", lever-only;
- the join door (`JoinRefusal::write`, the same arm shares the
  regularity arm's text): "whether two edges meeting at a vertex on one
  curve are one edge is undecided (…). Recourse: move the vertex clear
  of the pole, apex or tangency it sits near, or, if this size is
  intended, tighten the tolerance below 3e-10 m": sized, on the
  regularity reading's lever, which names no ruling.

`[tolerance.recourse]` (i): one message, one recourse per decision. Ev
on `[ev]` PR 5113 (fork-log row 144): a structural act "refuses in band
sized at the result's gate". The boolean's lever-only ending is pinned
by `undecided_text::the_booleans_undecided_join_ends_once`, and the
doc comment on `JoinUndecided` argues it ("a form selection on no size
the user chose"), so this is a choice to confirm, not only a slip.
Neither ending is the kernel's defect.

## Done when

Both doors end the class reading with one text and one recourse.
