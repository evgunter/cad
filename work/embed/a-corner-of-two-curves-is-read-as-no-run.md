---
id: a-corner-of-two-curves-is-read-as-no-run
kind: issue
title: A corner where a curve gives way to another curve (an arc into an arc of another radius, two conics at a lens tip) is read as no run: the next edge would need the read curve's own runs, others and apart
status: open
opened: 2026-11-22
priority: P0
cost: M
refs: [a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation, departure-reads-no-curved-corner-off-its-shared-edge]
---

## Finding

From lane `embed/tangent-corner` and its review (PR 7320, 2026-11-22).

`topo::injective` reads a corner whose next edge continues its curve
at first order as one run through the vertex
(`present_corner`, "One run through the vertex"), where the next edge
is straight in the chart. Where it is another curve, `reading` gives
the corner no reading and `beside_curve` types it `Structural`.

The PR's first head read a curved next edge too, by a ladder of its
own (`curve_onward`): the edge's pieces walked from the vertex until
one was decided past the columns' far end. The review showed it
unsound: nothing read the edge after that parameter, and `alone`
admits the edge by index, so a next edge that turns and comes back
through the columns inside `R` was presented by one side. The fixture
is pinned as `injective::presentation_rows::a_curved_next_edge_that_comes_back_into_r`:
a plane face outside the unit circle's upper half, continued
tangentially at `(1, 0)` by a circle of radius 0.2 centred `(0.8, 0)`
over a clockwise half turn, then a line back; the face point
`(0.67, −0.16)` is in the cell `(0.66, 1.04) × (−0.18, 0.1)` on the
side that reading named as not the face. The ladder was removed.

## What it holds back

`band_closed_mixed_chain_shapes::a_four_arc_barrel_fillets_whole` (a
cap bounded by four arcs of two radii, each tangent to the next, no
line): at `f64` with the at-rest gate asking it refuses
`Uncertified { Departure }`, plane against torus across an arc. With
the removed ladder it held at `f64`. One census test; a keyhole or a
peanut outline is the same shape. The lily leaves' lens tips
(`departure-reads-no-curved-corner-off-its-shared-edge`, "a corner of
two curved edges of different conics") are this row's corner too.

## What is needed

The next edge read by the code that reads the first: its lifts cut
into runs, exactly one in reach, monotone across the columns from the
vertex to their far end, every other run apart from `R`
(`runs`, `monotone_run`, `apart`), not a second, shorter ladder. Then
`graph_run` already takes any carrier for the next edge.

## Acceptance, scope and what the build must hold (2026-11-23, PR 7320's re-review)

- **Acceptance.** `injective::presentation_rows::a_curved_next_edge_that_comes_back_into_r`
  stays unread (it is the unsound case: the next edge comes back
  inside `R`). The four-arc barrel comes back as a body row in
  `self_intersection_proves_injectivity`'s
  `a_support_face_c1_through_a_vertex_is_embedded_at_its_tangent_joints`
  (`Embedded` at `f64`, as built and in that row's poses). The two
  conic rows that pin `Structural` today
  (`two_arcs_of_one_circle_are_read_on_through_their_vertex`'s
  two-circle tip, `an_arc_of_one_circle_that_does_not_continue_the_shared_arc_is_not_read`'s
  over-a-turn case) re-pin to the run.
- **Census tests it holds back, measured on PR 7320's frozen head
  with the at-rest gate asking**: `band_closed_mixed_chain_shapes::a_four_arc_barrel_fillets_whole`
  and `band_open_mixed_chain_shapes::a_compound_corner_of_two_arcs_builds`,
  both `Uncertified { Departure }` plane against torus across an arc
  at `f64`. With the removed ladder the second passed and the first
  got past `f64` to refuse at `Interval`.
  Both come back when this unit is built: the compound corner to
  passing, the barrel past `f64`.
- **The mutant to hold.** The next edge read with no test that its
  other runs and lifts are apart from `R` (the removed ladder, in
  effect): red on `a_curved_next_edge_that_comes_back_into_r`.
- **A periodic chart is not in scope.** The removed ladder read no
  chart with a periodic axis, both census tests are plane faces, and
  no body with two curves at a tangent joint on a cylinder or torus
  is known. It stays unread, and gets its own row if a body shows it.
- **`graph_run` passes `closed = false` to `edge_piece` for the next
  edge** (`edge_piece(other, *other_span, false, &[image])`). Right
  for a line. A curved next edge that is a closed conic needs its
  seam read as the first edge's is, or to be refused there.
