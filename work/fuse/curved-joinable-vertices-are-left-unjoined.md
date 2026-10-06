---
id: curved-joinable-vertices-are-left-unjoined
kind: issue
title: The join takes planar joinable vertices only: curved valence-2 vertices (arcs of one rim circle, Chart seams of one surface) are left unjoined and unrefused
status: open
opened: 2026-10-06
priority: P1
cost: H
refs: [a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made, 4140]
---


## The finding

PR 4140 wires the join (`crates/topo/src/boolean/edge_join.rs`) into
every boolean output stage. `joinable` takes a vertex between two
planar faces whose edges are certified lines only (`edge_join::joinable`,
`certified_line`), so a valence-2 vertex between two curved edges of
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

## Ruled (Ev, PR 3881, 2026-10-03)

The ratified body already decides it; this row builds it, with no fork
open:

- "**Curved edges** join only where their carrier is structurally one,
  keyed by the surfaces and the intersection branch. Otherwise the op
  refuses typed."
- "A closed edge keeps the canonical cut fixed by its carrier (C12.5),
  which is a function of the carrier, not of history."
- "The theorem is proven for the planar inventory. Curved arms join it
  as each gains a structural intersection carrier."

## Where step 2 stands against it

Step 2 (PR 4140) neither joins nor refuses a curved valence-2 vertex:
an interim contrary to the ruling, which this row closes. Refusing
typed today would break hundreds of rows (the counts above). Until it
closes, a curved cut an earlier step made can still leave a result that
depends on member order, as the planar cut did before PR 4140.

## What the build needs

- **The branch key.** The `Intersection` description selects its
  branch by a witness point, so "these two arcs lie on one curve" has
  no key to compare. The join decides by keys and carriers only, so the
  description needs one.
- **The closed-edge join.** Two arcs of one circle join into a closed
  circle carrying its C12.5 canonical cut, fixed by its carrier.
- **The refusal** for a curved valence-2 vertex whose carrier is not
  structurally one, typed.

The DESIGN maximal-edges wording for the closed edge's cut lands with
this row.

## Order

This precedes step 3: until it lands, step 3's check
(`topo::joinable_vertices`) is planar-only. The partial-revolve half of
`sweeps-build-one-rim-edge-per-segment-not-per-run` also needs it.
