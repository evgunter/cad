---
id: a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex
kind: issue
title: A quarter round resting flush with a slab's edge or corner refuses its differences and intersection with an odd count of surviving crossing records at the rim vertex pair
status: closed
opened: 2026-10-10
priority: P1
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide, two-discs-kissing-externally-refuse-classification-invariant]
branch: contacthold/flush-quarter-round
pr: 4704
closed: 2026-10-11
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

## Measured

Instrumented on `contacthold/flush-quarter-round` (main at 6e3985cb76),
`flush`, undeclared, slab ∖ round, at the rim vertex pair `(2, 4, 1)`
(the slab's vertex split onto the round's rim vertex). The slab's
sectors are its side face `x = 2` and its top, each halved by a
bisector. The round's sectors are the wall (outward normal `−z` at the
rim), the cap halved by a bisector, and the bottom. `recl_edges` reads
two events:

- **the rim, `−y`**: a real edge of both solids, so
  `resolve_edge_edge`. The slab's top and the round's wall are a
  `Tangent` pair, declared or verified by the door undeclared, so the
  declared-`Tangent` short-circuit skipped the membership and marked
  the flanking record (slab side, round wall) as the germ, whatever the
  op;
- **the round's cap/bottom edge, `−x`**, in the slab's top: the
  edge-sector rule, Table II through the Eq. 15.3 lump of the coplanar
  top and bottom (opposite orientation). It is a germ for ∪ (the lump
  is In) and none for ∖ and ∩.

So ∪ held two survivors and ∖ and ∩ one, at both rim vertices: the odd
count. The arc's side never enters it.

## The fix

The tangent short-circuit only applies where the tangent faces leave
the common line on the same side, riding the locus together. The
slab's top leaves the rim along `−x` and the wall along `+x`, so they
meet only along the line, and the membership algebra decides. In it,
the slab top's representative lies on both of the round's flanking
planes: the wall's tangent plane is the bottom's plane, a half-turn
wedge. It points along the bottom and away from the wall, so the
bottom's lump decides and the wall's touch is no reading of it. Before
this, the split went to `wedge_is_reflex`, which refuses a half-turn at
a decided zero, the capsule's `bool_wedge_reflex`
(`the-capsule-rod-ending-on-the-joint-parts-by-member-order`).

The same-side tangent pairs the short-circuit still serves refuse at
every op where they only touch along the line. Whether to remove the
short-circuit waits on what a tangent touch's union should produce:
`a-same-side-tangent-touch-refuses-at-every-op`.

## Closed 2026-10-11 (PR 4704)

Merged after a holdout dual review (DR row file `PR-4704`). The same-side residue is `a-same-side-tangent-touch-refuses-at-every-op`, now with Ev in PR 4723.
