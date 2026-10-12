---
id: a-corner-held-as-several-result-vertices-ties
kind: issue
title: A corner the result holds as several vertices is named as one tied name over them, losing the per-vertex address main gave each
status: closed
opened: 2026-10-12
priority: P2
cost: M
closed: 2026-10-12
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

## Ruling (Ev, PR 4881, fork-log row 124)

Each vertex an operand corner is held as is a fragment of it:
- `From(x, v) + Fragment(Borders)` over the walls that divide the corner there, read as a face piece's walls are (`borders.rs`'s divides filter);
- equal sets tie;
- a corner held as one vertex keeps its bare name.

The build is this row's. It re-pins
`touch_reread_rows::a_corner_held_as_several_result_vertices_is_one_tied_name`.

## Closed

Built on branch `emit/corner-fragments`. `emit_topo::name_boolean_vertices`
names a corner the result holds as several vertices `From(x, v) +
Fragment(Borders)` at each copy, the walls read by
`borders::Obstacles::split_corner`: the corner's faces (its star in the
operand) are the parent, each copy's boundary is the half-edges at the
copy of the faces that hold part of them (merged faces included), and
the obstacles, the divides filter and the wall citation are
`Obstacles::split`'s own (one `walls` reading now serves both). A corner
held as one vertex keeps its bare name.

One change to the obstacle reading itself: discards of a parent's faces
are joined where both bounded one edge that died with them
(`Discard::bounds`, the first edge of each boundary chain). A face
parent has one face, so this joins only across an edge with that face
on both sides; a corner's parent has several, and its discarded region
crosses the corner's own edges. Without it the divides filter emptied
both copies' sets in 5 of the 36 groups ("near-flat quad void -1e-3
buried, island, hang_over", `x ∩ y`, every pose): each of three
discards bordered one copy. With it no set is empty. The full
`editor-core` suite (2976 tests, default ε) moved no face name.

Measured, `touch_reread_rows`: 36 tied groups before (the single-scene
row, plus 7 corners × 5 poses in the every-op rows: 25 of two copies,
10 of three), all 36 separate now, none empty.
`a_corner_held_as_several_result_vertices_is_one_tied_name` is
re-pinned as `…_is_a_fragment_at_each` (two fragments of `x`'s apex,
both citing `y`'s faces, the sets apart, no bare name), and the
every-op rows now assert 7 fragmented corners per pose and no corner
name that ties.

The union's `Flush::least_vertex` path needs no fragment: a finished
union's neighbourhood at a point is the union of its members', so a
member corner lies in one finished vertex. Not changed.
