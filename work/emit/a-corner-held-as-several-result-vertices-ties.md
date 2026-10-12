---
id: a-corner-held-as-several-result-vertices-ties
kind: issue
title: A corner the result holds as several vertices is named as one tied name over them, losing the per-vertex address main gave each
status: open
opened: 2026-10-12
priority: P2
cost: M
design: true
needs_ev: true
---



## What

The touch ruling (Ev, PR 4284) says: "A vertex of either operand is
named as that vertex, wherever the other operand meets it". The build
(PR 4816) reads that identity at every operand key the boolean leaves
at the point, null copies included (`emit_topo::operand_vertex_keys`).
Where the result keeps several vertices at one operand corner, each a
null copy of it, all of them are that corner, so the name is a tie over
them (`name_boolean_vertices`, the `corners` map → `mint_candidates`).

**Measured.** `corners` → `mint_candidates` ties fire 35 times, all in
`touch_reread_rows::a_vertex_read_twice_names_its_result_in_every_op`
and `…_at_every_pose`. An example is "over a void in the bare arch",
`x − y` at rest. A pyramid stands on its apex in the apex of a pyramid
less a nested pyramidal void, and the result holds two vertices at
`x`'s apex.
- main (acffb186a5): two distinct unique names. The kept key is `x`'s
  `CapVertex` and the other is a `Seam` of two parents (the touch
  fallback). Each vertex is addressable, but one of them is a corner
  spelled as a seam.
- PR 4816: both are `From(x, CapVertex …)`, one tied row with two
  candidates. Pinned by
  `touch_reread_rows::a_corner_held_as_several_result_vertices_is_one_tied_name`.

Nothing is wrong geometrically. A reference to that corner is now
offered both candidates, where main resolved one of them uniquely.

## The question

Do several result vertices at one operand corner owe a discriminator
(for example which cone of the corner each holds, or the faces around
each), or is the tie the honest name? Main's per-vertex address came
from spelling one of them as a seam, which the ruling retires. A design
fork, not to be settled by inventing a discriminator in the emitter.
