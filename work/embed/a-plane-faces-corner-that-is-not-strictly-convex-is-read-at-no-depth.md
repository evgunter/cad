---
id: a-plane-faces-corner-that-is-not-strictly-convex-is-read-at-no-depth
kind: issue
title: present_plane reads a two-line corner only where it is strictly convex, so a vertex on a straight run of a plane face's boundary, or a reflex corner, is read by no presentation at any depth and a valid body refuses at its door
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

Traced by the planar lane of the per-solid census
(`embed/trace-planar`, 2026-11-21), on `embed/gate-measure` at
`1ca743418c` with diagnostic exit lines (`embed/trace-planar-measure`).

`topo::injective`'s `present_plane` (`crates/topo/src/injective.rs`)
reads a plane face's part of a cell in two shapes only: one line edge
(the `[e]` arm), or two line edges at a vertex whose corner is strictly
convex (the `[e1, e2]` arm). The second arm asks both `lean(n1, bv)` and
`lean(n2, a)` to decide `Positive` at `SELF_INTERSECTION_CORNER`, and
returns `Unpresented::Local` otherwise. Two corners of a valid face fail
that gate, and every smaller cell holding the vertex holds the same two
edges, so they fail it at every depth:

- **a straight corner**, a vertex in the middle of one straight run of
  the face's boundary. A Boolean leaves one wherever it splits a face's
  edge at a vertex of the other operand. Both leans are exactly 0 where
  the face's chart holds one coordinate fixed along both edges (an
  axis-aligned body); on any other placement each is a rounding wide and
  decides `Zero` at the band. The exact ones are read since 2026-11-22
  (the section at the end); the others are not.
- **a reflex corner**. Both leans are negative. That the arm does not
  present a reflex corner as its quadrant is right, and
  `a_planes_reflex_corner_of_two_lines_is_not_its_wedge` holds it. But
  no other presentation reads that corner either.

When the pair's shared cell is that vertex, or an edge ending there, the
pair is `Split { Departure }` down to the depth floor and the walk
refuses it as `Uncertified { arm: Departure }`.

## Trace

Cheapest: `topo::all review_m9_1_probes::probe_records_partialeq_bites_on_mutation`
(0.11 s). It is the union of the bricks `[0,1]³` and
`[0.5,1.5] × [0.25,1.25] × [1,2]`, which is one valid solid. The gate
refuses `Uncertified { a: 4v1, b: 10v1, shared: Vertex(14v1), arm: Departure }`.
Here vertex 14 is `(1, 0.25, 1)`, face 4 is `a`'s side `x = 1` and face
10 is `b`'s side `y = 0.25`. The two walls meet at that point only. On
each of them the point splits a straight edge in two (face 4's top edge,
and face 10's bottom edge), so each face has a straight corner there.
Of the 152 asks of that pair, 134 exit at the lean gate on both faces.
The rest have more than two edges in the cell, coarser. At the finest
cells (`u` width 2⁻¹⁰) every lean the gate read was exactly
`0.000e0`: 627 straight-corner exits in the test, against 4 reflex ones.

## Count

On the census of main `c2a0c41f40`:

- **departure, vertex, plane/plane: 57 of the 96 traced** (of 101).
  The refusing pair's finest cells exit at the lean gate on one face or
  both. Re-run for the lean values: 43 tests meet straight corners
  (lean exactly 0) and 10 meet only reflex ones. The other 4 are the
  slow Boolean corpus rows and were not re-run.
- **departure, edge, plane/plane: 32 of the 111 traced** (of 116).
  These exit at the same gate at a shared edge's end, on one face or
  both. Their lean values were not re-run.

- **`Budget(Depth)`, probably.** In 2 of the 10 depth-bucket tests
  re-run, the deepest touching pairs exit at the lean gate. In
  `verbs_f7_r2_probes::r2_attack_midvertex_chord_split` that is 251 of
  413 asks, and in `holes_meeting_at_a_vertex::runs_one_between_others_build_in_every_op`
  59 and 34 of 106. The census reads `Budget(Depth)` there because a
  distance pair beside them reaches the floor first. Not checked by a
  flip.

The families are the flush unions and their naming rows
(`emit_union_rim_piece_ranks`, `emit_seam_junction`,
`union_flush_onto_edge_contact`), the slits and keels
(`a_slit_tip_on_a_curved_face_reads_its_full_turn`,
`a_keel_on_a_bores_ruling`, `a_keel_on_the_floor_subtracts_as_a_slit`),
`m9_1` and `m3_pr6` records, `three_solids_on_one_line` and
`split_pinch_keeps`. Every one is an ordinary Boolean result.

## What a fix needs

The straight corner is one line, but the body does not record that: a
Boolean stores the two pieces of a split edge as two lines, each from
its own origin and opposed, so only their chart points say they are
one. Read as one run, the `[e]` arm's half-plane presents it (the
section at the end says what landed and what is still owed). The reflex
corner is a union of two half-planes, not their
intersection. It wants a presentation of its own, or a split of the
cell at the vertex into two convex sectors, each read as a convex
corner against the partner. The quadrant reading stays refused
(`a_planes_reflex_corner_of_two_lines_is_not_its_wedge`).

Cheapest edge reproducer:
`topo::all records_cite_their_decision::a_cell_reached_from_both_sides_cites_its_shortest_chain`
(0.12 s).

## Measured and landed: the exact straight corner (embed/straight-corner, 2026-11-22)

Main `9bc9d6517`, gate and trace harness merged on
`embed/straight-corner-measure`. Population: the census's plane/plane
`Uncertified/Departure` tests, vertex and edge, 217 in the JSON, 215
run (one is `pncad`, one no longer exists by that name). f64 except 4.

**What the body holds.** In the flush union of two bricks
(`review_m9_1_probes::probe_records_partialeq_bites_on_mutation`) face
4's two edges at vertex 14 are `Line { origin (1,0,1), dir +y }` over
`(0, 0.25)` and `Line { origin (1,1,1), dir -y }` over `(0, 0.75)`:
two records, two origins, opposed directions. A reading of "one stored
line" (same origin and direction) was built first and moved 3 of 215,
so it is not what landed.

**What landed.** `present_plane` reads two line edges as one run where
each edge's far end lies on the other's line as the point 0 (the lean's
enclosure is exactly `[0, 0]`; no band is asked) and the two leave the
vertex opposite ways, one into it and one out. The run presents its
half-plane. The zero is the side on both edges where the pair shares
both, a side past the shared cell (`Zero::Past`) where it shares one,
and the face's own run through the vertex (`Zero::Through`) where it
shares the vertex alone. `planes_through` builds the plane for the
last: the two runs' plane where both faces have one, else a plane
through the one run, turned to bisect the faces' headings.
`Zero::Through` is read only under a plane that lists the run's edges
and holds whole lines, against a partner whose zero is nothing, a
corner point or its own run.

**Census, before and after.** Tests that pass with the gate asking: 0
of 215 before, 24 after. By each test's first non-`Embedded` ask:

| before | after | tests |
|---|---|---|
| departure, vertex | every ask `Embedded` | 17 |
| departure, edge | every ask `Embedded` | 5 |
| departure, vertex | `Crosses`, covered by the door's records (a slit or keel body later in the test, reached now that the earlier body holds) | 15 |
| departure, vertex | departure, edge, plane/plane | 5 |
| departure, edge or vertex | departure, cylinder/plane | 5 |
| departure, vertex | `Budget(Depth)` | 1 |
| departure, vertex | departure, vertex, plane/plane | 61 |
| departure, edge | departure, edge, plane/plane | 106 |

**Why the rest still refuse** (a probe at the lean gate over the 179
tests that still refused, each lean classed exact 0, `Zero` at the
band, negative or undecided):

- **78 exit only at reflex corners** (both leans negative). The trace
  counted 10 of the 57 vertex tests it re-ran. The reflex corner is the
  larger part of this row, not the smaller.
- **32 exit at corners straight only to rounding**: both leans decide
  `Zero` at the band and neither is exactly 0. The widest enclosure in
  any of them is 3.3e-14 m, against ε = 1e-9 m. These are bodies off
  the axes, or built through a chain that rounds.
- **66 exit at no two-line corner at all**: other causes (more than two
  edges in the cell, thin faces, the vertex bisector).
- 3 exited at exact-zero leans because the first cut also asked that
  the two edges meet at one chart point; that condition was dropped
  (the half-plane is the line's wherever each edge's end falls on it).

**Orientation.** The flush union holds at both scalars as built and
moved a hundred units off. Turned by 0.7 rad about `(2, 3, 6)/7` it
refuses `Budget(Depth)` at both scalars, 20 pairs refused: its runs'
leans are a rounding wide. A mutant reading the run at the band's
`Zero` instead of the point 0 makes that turned body hold.

## Owed

- **The straight corner off the axes** (32 census tests, and every
  turned body). Reading it needs a decision at a scale: the lean
  decided `Zero` at `SELF_INTERSECTION_CORNER`. That would present a
  half-plane whose line is within the band of the two edges and not on
  them, drop the face's points between, and list edges under a
  separator that holds them only within the band. It changes what `On`
  promises (membership, exact) and adds a reader of a band-scale drop,
  so it is a design question, with the sliver's and the margin's.
- **The reflex corner** (78 tests exit only there). The face at any
  two-line corner is the sector from one edge to the other, and a ray
  into the face from the vertex splits it into two convex sectors, each
  the image of a rectangle under `Map::Affine`, for a reflex, straight
  or convex corner alike. That is two presentations of one face's part:
  `Presentation` gains a pair, and `departure`, its carrier arm,
  `opposite`, `planes_through`' headings, `graph_of`, `cusp` and
  `stuck` each read one today. A straight corner cannot be closed this
  way where the vertex alone is shared: a plane through the vertex has
  to be strictly positive along both edges, which leave it opposite
  ways.

