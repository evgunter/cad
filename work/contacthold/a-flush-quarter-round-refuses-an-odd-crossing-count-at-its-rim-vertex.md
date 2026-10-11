---
id: a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex
kind: issue
title: A quarter round resting flush with a slab's edge or corner refuses its differences and intersection with an odd count of surviving crossing records at the rim vertex pair
status: open
opened: 2026-10-10
priority: P1
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide, two-discs-kissing-externally-refuse-classification-invariant]
---


## Finding

`crates/sweep/tests/an_arc_tangent_at_a_boolean_vertex.rs`'s
`the_flush_and_corner_postures` (print-only; run
`cargo test -p sweep --test all the_flush_and_corner_postures -- --ignored --nocapture`)
rests a radius-1 quarter round (x ∈ [0.5, 3], z ∈ [1, 3], y ∈ [0, 4],
its bottom face tangent to its wall along the rim x = 2, z = 1) on a
slab whose top z = 1 ends at the rim:

- `flush`: slab x ∈ [0, 2], y ∈ [−1, 5] — the rim vertices lie on the
  slab's top edge;
- `corner`: slab x ∈ [0, 2], y ∈ [0, 4] — the rim vertices are slab
  corners.

Undeclared, with the resting faces declared `Rest`, and with the wall
declared `Tangent` too, the union answers (volume `vs + vr`), and slab
∖ round, round ∖ slab and slab ∩ round all refuse
`ClassificationInvariant { what: "odd number of surviving crossing
records at a vertex pair" }`, raised in `boolean::insert::plan_null_pairs`
(the vertex-vertex pass). The right answers are the slab, the round and
`Empty`.

**Not the arc's side.** Measured on `contacthold/side-code-measure`
(main at 7e607dded9): forcing the cap arcs' side codes to their true side
(their far vertex, `Reach::Chord`) moved none of these refusals, and the
second-order reading that landed with
`boolean-conic-side-code-zero-is-first-order` leaves them as they were.
The cause is unmeasured: read the surviving records at the rim vertex
pair before building against a theory of it.

Before INTENT's E unit (PR 4496) the undeclared postures refused
`UndeclaredCoincidence` (`bool_plane_offset`) first, and `flush` with
`Rest + Tangent` was already this refusal (main 493bf8b).

Possibly related, not checked against this fixture:
`work/sector/a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide.md`,
`work/tang/two-discs-kissing-externally-refuse-classification-invariant.md`.
