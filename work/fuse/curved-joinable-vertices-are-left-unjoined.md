---
id: curved-joinable-vertices-are-left-unjoined
kind: issue
title: The join takes planar joinable vertices only: curved valence-2 vertices (arcs of one rim circle, Chart seams of one surface) are left unjoined and unrefused
status: open
opened: 2026-10-06
priority: P1
cost: H
design: true
refs: [a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made, 4140]
---


## The finding

PR 4140 wires the join (`crates/topo/src/boolean/edge_join.rs`) into
every boolean output stage. `joinable` takes line edges only
(`edge_join.rs:115`), so a valence-2 vertex between two curved edges of
one face pair stays: neither joined nor refused.

Measured by a probe over every boolean output in the workspace `ci`
suite. It logged each valence-2 vertex between one face pair that the
join did not take, per output, so repeats count again:

- **planar ones left: 0** — the planar join is complete on the suite;
- 1290 cylinder|plane arcs of one rim circle (an `Intersection` of
  one surface pair);
- 100 plane|sphere arcs; 30 sphere|sphere arcs;
- 49049 sphere|sphere, 386 cylinder|cylinder rulings and 48 circles,
  all `Chart`-described seams of two faces on one surface;
- small counts on cone, torus and tangent pairs.

## Why it is not refused

Refusing typed would break hundreds of rows. Left as is, a curved cut
an earlier step made can still leave a result that depends on member
order, as the planar cut did before PR 4140.

## The design question

Joining a curved run needs:
- **closed-edge handling:** two arcs of one circle join into a closed
  circle, which must carry its C12.5 cut;
- **a branch key:** the `Intersection` description selects its branch
  by a witness point, so "these two arcs lie on one curve" has no key
  to compare. The join decides by keys and carriers only, so it needs
  one.

## Order

This precedes step 3: until it lands, step 3's check
(`topo::joinable_vertices`) is planar-only. The partial-revolve half of
`sweeps-build-one-rim-edge-per-segment-not-per-run` also needs it.
