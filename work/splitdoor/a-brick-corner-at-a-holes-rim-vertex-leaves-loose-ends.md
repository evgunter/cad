---
id: a-brick-corner-at-a-holes-rim-vertex-leaves-loose-ends
kind: issue
title: A brick whose corner stands on a hole's rim vertex refuses UnpairedLooseEnds at every tilt
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by the sweep of
`a-boolean-whose-face-is-tangent-to-a-holes-rim-at-a-vertex-refuses-its-strut-order`
(branch `splitdoor/boolean-strut-order`), looking for a pose that
reaches the orbit walk's germ tie
(`the-booleans-orbit-walk-orders-two-germs-along-one-direction-at-first-order-alone`).

## What

A = `holed_block(π/2, 2π)` or `two_arc_holed_block(π/2, 2π)` from
`crates/sweep/tests/a_split_through_a_wrap_edges_vertex.rs`, turned
about x by `t ∈ {±0.4, ±0.9}`; its rim vertex lands at
`v = (0, cos t, sin t)`. B = `brick((-6, 0), (-6, 0), (-6, 0))`
translated by `v` (its corner on the vertex), then turned about the
vertical through `v` by `spin ∈ {±π/4, ±0.3, ±2.5}`. A − B, A ∩ B and
A ∪ B, `SweepStrategy::Realized`, ε = 1e-9: 144 runs, identical on
`main` (4f29a37c) and on the branch.

- 69 refuse `Join(BooleanJoin(UnpairedLooseEnds { count: 2 }))` and 3
  `Join(BooleanJoin(SectionLoopMixed { .. }))` (two arcs, `t = 0.9`,
  `spin = −0.3`), leaning onto the vertex and away alike;
- 28 build at tier 3 and fail tier 3′ over their contacts with
  `CensusUnsupported { cause: ContactLane(NotCertifiable { what: "the
  meeting reading of two curved edges could not settle a count" }) }`;
- 32 build at tier 3′, 12 build empty. No volume was checked.

Unread: which join site leaves the loose ends, and whether the
3′ census rows are the contact lane's known limit or a wrong body.
