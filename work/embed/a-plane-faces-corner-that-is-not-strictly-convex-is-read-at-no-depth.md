---
id: a-plane-faces-corner-that-is-not-strictly-convex-is-read-at-no-depth
kind: issue
title: present_plane reads a two-line corner only where it is strictly convex, so a vertex on a straight run of a plane face's boundary, or a reflex corner, is read by no presentation at any depth and a valid body refuses at its door
status: dispatched
opened: 2026-11-21
priority: P0
cost: M
branch: embed/corner-pieces
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
  edge at a vertex of the other operand. Both leans are exactly 0 only
  where every chart operation on the two edges is exact in floating
  point, which is dyadic coordinates, not axis alignment (PR 7338's
  review); anywhere else each is a rounding wide and decides `Zero` at
  the band. A reading of the exact ones was built and dropped (the
  sections at the end).
- **a reflex corner**. Both leans are negative. That the arm does not
  present a reflex corner as its quadrant is right, and
  `a_planes_reflex_corner_of_two_lines_is_not_its_wedge` holds it. No
  other presentation reads that corner either, which is true and is not
  the defect (`embed/iso-end`, 2026-11-22): a reflex face is read
  through its own carrier, by the carrier arm, which reads the PARTNER's
  presentation. So a pair at a reflex corner refuses where the partner
  is not presented, or is presented as a sliver. A straight corner on
  the partner is one such exit.

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

## Measured, built and dropped: the exact straight corner (embed/straight-corner, PR 7338, 2026-11-22)

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

**What was built.** `present_plane` read two line edges as one run where
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
of 215 before, 24 after. 22 of the 24 have every ask `Embedded`; the
other 2 pass with an ask that is not, which their test tolerates. By
each test's first non-`Embedded` ask:

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

- **78 have reflex exits and no straight-in-band ones** (both leans
  negative): 71 show only reflex exits and 7 also show exact-zero ones
  (PR 7338's review). The trace counted 10 of the 57 vertex tests it
  re-ran. The reflex corner is the larger part of this row, not the
  smaller.
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

**Why it was dropped** (PR 7338's review at `815d12e37`, probes on
`origin/embed/straight-corner-review`; no unsoundness found in a body).
The reading's reach is dyadic placements. The flush union moved by
`(0.5, 0.25, 0.125)` or `(3, 0, 0)` holds at both scalars; moved by
`(0.1, 0.2, 0.3)`, built there, or with sides 0.7, it refuses at both;
moved by `(1/3, 10.1, −7.7)` it holds at `f64` and refuses at
`Interval`, a valid body in two verdict classes where main had both
refusing. It also added a zero variant, a reader and two plane arms the
design below deletes, and its two-run plane lists edges it holds only
where two stored lines meet
(`work/embed/the-two-line-plane-lists-a-second-edge-it-holds-only-where-the-stored-lines-meet.md`).
The PR does not land; its branch stays.

## Owed

**Design: a two-line corner read as two convex pieces**
(`embed/corner-pieces`, 2026-11-22; built since, see "Built" below,
which says where the build departs from this design). It takes
the straight corner at any placement, the reflex corner and every
turned body as one reading. The
band-scale reading considered before it (a lean decided `Zero` at
`SELF_INTERSECTION_CORNER`, a line held within ε, edges listed under a
separator that holds them within ε) is not needed and is not taken.

### The reading

A plane face's part of a cell that holds exactly two line edges `e₁`,
`e₂` at one vertex `V` is presented as two pieces. With `a`, `b` the
edges' directions from `V` and `n₁`, `n₂` their toward-face normals in
the chart, the ray is `m = n₁/|n₁| + n₂/|n₂|`: the bisector into the
face for a convex and a reflex corner alike, the common normal for a
straight one. The pieces are the cones of `(a, m)` and of `(b, m)`,
each the image of a rectangle `[0, x̄] × [0, ȳ]` under `Map::Affine`
with origin `V`'s chart point. Each piece has its own zero: `Side` on
its edge where the pair shares that edge, else `Point` at `V` where the
pair shares `V`, else `None`. The side along `m` is never a zero.

### Why it is sound, premise by premise

1. **The two pieces cover the face's part of the cell.** The face near
   `V` is the sector from `a` round to `b` on the face's side. A ray
   strictly inside that sector cuts it into the sectors `(a, m)` and
   `(m, b)`, whatever the corner's angle short of a full turn. Each
   piece must itself be convex (under a half turn), so toward 360°, a
   slit, no ray serves and the cell refuses. Guard:
   `m` strictly inside on both sides, which is two strict signs, the
   chart determinants `a × m` and `m × b` on the face's orientation,
   decided `Positive` or the cell refuses `Local`. They are levered as
   the lean is today (a length times a sine, metres, at
   `SELF_INTERSECTION_CORNER`: the same site and scale, a strict sign
   where today's gate also asks one; no `Zero` is read). At a spike
   (0° or 360° within the band) `m` vanishes or lies along an edge and
   the cell refuses.
2. **Nothing else bounds the face in the cell.** Guards, all existing:
   the cell's local edges are exactly `e₁` and `e₂`, both lines with one
   use each; its local vertices are `V` or none; the cell's reach along
   each edge stays inside the edge's far end (`SELF_INTERSECTION_SPAN`).
   A third edge in reach (a cap across a thin face, a hole's edge, the
   loop passing `V` twice) is not this arm: it refuses, or is
   `embed/strip`'s capped reading, which must not reach the pieces'
   exits. No bound is asked along `m`: with those guards the plane's
   disk in the ball meets the boundary only in `e₁ ∪ e₂`.
3. **A piece is `Vacuous` only after 1 and 2 hold**, and only by a
   certified coordinate: the cell lies wholly off the piece's cone
   (`x̄ ≤ 0` or `ȳ ≤ 0` with the piece's determinant decided). The FACE
   is `Vacuous`, which `departure` turns into `Certified`, only where
   both pieces are. One vacuous piece only drops its piece pairs. This
   is the order `embed/strip`'s review found broken there: the exit
   comes after the reading has said what bounds the face in the cell.
4. **A piece pair is certified as a corner is today.** Each piece is a
   convex corner to every reader: under a separator `ψ` it is strictly
   one-signed off its zero (a side: the derivative across; a point: the
   two derivatives along its axes, one common sign), and the partner's
   piece the other sign, so the two pieces meet only on their zeros,
   which lie on the shared cell.
5. **The face pair is certified only where every piece pair is** (those
   not dropped by 3). A coincidence of the two faces in the cell pair
   is a coincidence of some piece of one with some piece of the other,
   by 1.
6. **What is conceded** is what `Zero::Point` concedes today: the
   stored vertex against the edges' own ends. No lean is read at the
   band, no line is held within ε, and `On` lists only cells a
   separator holds by construction.

Completeness, not soundness: two closed pointed cones with no common
ray are strictly separable through the apex (Gordan), and
`parting_normal` (PR 7318) finds such a plane. A piece is a pointed
cone, so a piece pair refuses only where the pieces share a ray, which
is a real overlap of the faces, or where the margin is in the band.

### What each reader becomes

| reader | today | after | why nothing new is certified wrongly |
|---|---|---|---|
| `Presentation` | `Vacuous`, or one `Presented` | `Vacuous`, or one or two `Presented` | a type change; one-piece readings are unchanged |
| `present_plane`, two lines | the lean gate, then a quadrant; the exact run | the two pieces; the lean gate, the quadrant, the run go | premises 1 to 3 |
| `departure` | the carrier arm, `planes_through`, `graph_of`, `cusp`, once per face pair | the same ladder once per piece pair; certified iff every piece pair is; any piece pair's cusp is the pair's | premise 5; each rung reads one piece as it reads a corner today |
| the carrier arm | the other face's one presentation against this carrier | each of the other face's pieces against it | a piece is a subset of the face |
| `opposite` | two `sign_on` calls | unchanged, per piece pair | |
| `planes_through` | built from `near` and two presentations | built from the faces' `near` and the two PIECES (as built; the design gave each piece pair a near of its own, which was dropped) | a plane through a shared edge lists the edge's ends, so it holds the corner of a piece off that edge; where that plane is zero along the off-edge piece too (a straight corner, `n·b = 0`) it parts nothing and the pair is left to the carrier arm, which closes every valid plane/plane pair that shares one edge |
| `anchor`, `heading`, `wedges_apart` | per presentation | per piece | they choose a direction and decide nothing |
| `corner` | either presentation's | any piece's | a flag |
| `cusp` | the two shared normals | the pieces that hold the shared edge | the others have no shared side |
| `stuck` | `Structural`, or a past curve | unchanged | a plane piece is neither |
| `past_read` | the partner's zero is on the shared cell | unchanged; its plane producer (the run's `Zero::Past`) goes | |
| `sign_on` | one arm per zero | no new arm; `Zero::Through` and `through_read` go | fewer readings |
| `arm_of` (PR 7329) | per face pair, by `near` | per piece pair, by that piece pair's near | below |

### The margin (PR 7329, read at `origin/embed/remeter`)

There a derivative is decided as a sine, levered at `arm_of`: the
faces' reach over a shared edge, or from a shared vertex. A piece pair
that shares only `V` inside a face pair that shares `e₁` takes the
vertex arm, the faces' reach from `V`. A piece is not a face, but the
arm is the faces': the displacement an angle error causes is over the
face, not over the piece. A thin or oblique piece loses nothing under
that metering: the sine is free of the rectangle's extent, and for a
convex corner a piece pair's best sine is at least the face pair's,
since each piece's cone lies in its face's. For a straight or a reflex
corner there is no face-pair sine to lose. Under main's present
metering (a derivative times the rectangle's span) a piece is metered
as a corner's quadrant is today.

### Pinned and measured

`embed/corner-pieces-measure`, on PR 7318's separator. Each piece is
built as a convex corner whose second edge is the ray, so today's
`departure` reads a piece pair with no new code:

- `probe_pieces_sharing_the_vertex_alone`: a face exactly straight,
  convex and reflex by 1e-13, reflex at 207° and at 260°, against a
  straight partner and a reflex one: four of four piece pairs certified
  in every row, asked either way. Against a partner lying along the
  face, exactly the two piece pairs that share a ray are refused. On
  main's heading plane the two reflex rows read two of four.
- `probe_pieces_sharing_an_edge_of_the_corner`: a split shared edge,
  both faces straight: the pieces on edge 1 and on edge 2 certify
  across their edge, and the two cross pairs, which share `V` only,
  certify under a plane through `V`. A partner folded onto the face is
  refused (a cusp).

**Cost.** Counted on `embed/straight-corner-measure` (the exact run
built, the gate asking), over the census's 215 plane/plane departure
tests: 9,698,180 `departure` calls, of which 33.6% have a face that
would be two pieces; asked per piece pair they would be 13,845,880,
1.43 times as many (per test: median 1.37, worst 2.49). That is an
upper bound on asks of the ladder per call, taken under a build where
the corners the pieces would certify still split to the depth floor, so
the calls themselves should fall; that was not measured. A piece pair
is not a cell pair: the receipt's pairs and `Budget(Pairs)` do not
move by it.

How many of the 78 reflex tests the pieces close that the carrier arm
does not already is not measured and cannot be before the build: it
turns on whether each refusing pair's PARTNER is a two-line corner the
pieces now present (`embed/iso-end` counts 18 of its 32 reflex pairs
with an unpresented partner and 14 with a sliver one, which is
`embed/remeter`'s). The build measures it through the gate.

### What goes, and its pins

- The lean gate and the convex quadrant: its rows
  (`a_planes_reflex_corner_of_two_lines_is_not_its_wedge` and the
  convex-corner rows) are re-pinned on the pieces: a convex corner
  presents two pieces whose union is its quadrant, a reflex one two
  pieces that cover its three quadrants, never the wedge.
- PR 7338 does not land, so the run, `Zero::Through`, `through_read`
  and its plane arms are not on the base and nothing of them is
  deleted. Its rows are re-created here where they still say something:
  the band's-edge row becomes "a spike within the band refuses"; the
  flush-union, turned and flap body rows come over as below.

### Rows the unit owes

Holding at BOTH scalars, each through `topo::clearance::embedded`: the
flush union as built; moved by `(0.1, 0.2, 0.3)` and by
`(1/3, 10.1, −7.7)`; built at `(0.1, 0.2, 0.3)`; with sides 0.7; turned
about a generic axis; mirrored; at the session box's wall. These are
the rows the stopgap failed.

Never certified, or `Violated` (the review's partners and flaps): a
partner's edge along the run either way; a run on the same line in
another plane; a coplanar partner straddling the line; a steep
crossing; the flap through the run at its vertex. Certified, rightly: a
coplanar partner across the line. And one hazard the pieces must not
open (`review_m8_a_past_side_is_not_read_in_the_carrier_arm`): a wall's
edge lying in a reflex floor's interior, read by the carrier arm
against the floor's carrier, must stay `Split`. With pieces the floor
IS presented, so that pair is asked piece by piece: the floor's piece
that holds the wall's second edge in its interior shares a ray with the
wall's piece on that edge and is not parted. That is argued, not
probed: the unit pins it.

### The two-line plane's gap

A plane face's piece holds at most one edge of its corner, so a piece
pair with a plane piece never shares two line edges and never reaches
`planes_through`'s two-line arm: each piece pair's separator lists at
most that pair's one shared edge, which it holds by construction. The
arm keeps its gap for pairs with no plane piece; that is filed
(`work/embed/the-two-line-plane-lists-a-second-edge-it-holds-only-where-the-stored-lines-meet.md`).

### Not covered

- A curved chart's reflex corner (`embed/iso-end`).
- A cell holding a third edge (a cap, a hole, a pinch): `embed/strip`.
- A line meeting a curve (`embed/tangent-corner`).

## Census tests whose partner stops at the lean gate (EMBED lane `embed/iso-end`, 2026-11-22)

A plane face's reflex corner is read by the face's own carrier in the carrier arm, against the partner's presentation, where the partner is presented and lies on one side of the face's plane in the cell; no other first-order separator signs the face's part there (`a-cell-past-an-iso-edges-end-is-clipped-by-its-whole-line`, What a fix needs). Across a shared edge that holds for a valid body; at a vertex alone a valid partner may cross the plane in the quadrant the face lacks, and there the reflex half of this row stands (`embed/corner-pieces`). The census refusals below are partner readings.

**Cylinder bucket, 6 tests.** The cylinder's pair now certifies, and the test refuses at a plane pair whose face exits at `present_plane`'s lean gate:

- `editor-core::all emit_curved_rim_chord::a_cavity_cut_by_the_notched_prism_names_its_rim_pieces`
- `editor-core::all emit_curved_rim_chord::a_post_across_the_other_piece_of_a_notched_circle_unites`
- `editor-core::all wire_legal_union_refusals::two_bars_that_sever_a_sunk_boss_wall_only_together_name_the_glue`
- `editor-core::all wire_legal_union_refusals::two_blocks_that_open_a_sunk_boss_wall_only_together_name_the_glue`
- `editor-core::all emit_union_curved_flush::a_wall_across_two_flush_arcs_publishes_one_table_in_every_order`
- `editor-core::all wire_legal_union_refusals::a_chord_across_the_glued_pieces_line_names_wherever_the_cut_crosses`

**Plane/plane departure census.** Here the refusing pair holds a reflex lean (both < 0) on one face. Of 212 tests (vertex and edge buckets, ≤30 s each) on main `a29fd3695`, with the trace prints on `embed/iso-end-measure`, 31 do. In 13 of them the partner is unpresented (a straight corner, leans exactly 0, or another exit; not classed per test):

- `editor-core::all emit_union_borders::two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle`
- `step-import::all onb_wild_normal_census::no_wild_face_sits_on_the_frame_seam`
- `step-import::all wild::wild_bodies_are_a_fixed_point_of_our_own_dialect`
- `step-import::all wild::wild_files_import_and_agree_with_the_oracle`
- `sweep::all a_long_bound_beside_a_short_arm::a_boss_beside_a_holes_long_edge_at_every_angle`
- `sweep::all a_long_bound_beside_a_short_arm::a_boss_poking_out_of_a_hole`
- `sweep::all join2_r2_probes::a_channel_whose_arm_ends_join_two_ring_vertices_builds_sound`
- `sweep::all join2_r2_probes::an_island_of_ring_vertex_segments_builds_sound`
- `sweep::all reach_continuation::a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order`
- `sweep::all rest_zip_admission::a_box_dipping_into_a_plate_at_a_tangent_site_never_ships_the_overlap_twice`
- `sweep::all rest_zip_admission::the_tangent_site_keeps_building_pure_contacts`
- `topo::all a_corner_flat_at_its_short_bounds::a_long_edge_beside_a_corner_flat_at_its_short_bounds_is_classed_where_it_runs`
- `topo::all a_reflex_corner_pierced_through_a_plate::a_reflex_corner_whose_two_pieces_stand_out_of_a_plate_builds_or_refuses_typed`

## 2026-11-22: what the vertex-alone separator reads of a straight corner (embed/vertex-bisector)

At a vertex two faces share alone, `planes_through` parts two quadrant
presentations (`Zero::Point` on both) by the wedge plane
(`wedges_apart`): the point nearest the origin of the hull of the two
corners' axes, one corner's negated. A straight corner presented as a
quadrant has antiparallel axes (`a₁ = −a₂`), so the origin is in that
hull whatever the partner is, and the wedge plane never parts it. A
straight corner read through the `[e]` arm's half-plane at a vertex
alone needs a separator of its own there: one through the corner's
line, leaning off the partner, which the one-line arm builds only
where the line is a shared edge.

## Built: the two pieces (embed/corner-pieces, 2026-11-23)

Built on `origin/embed/wave3-land`, last merged at `7f85375799`, the landing branch,
which holds PR 7329's margin (`embed/remeter` `7667c455e`) and PR
7318's separator. It needs both.

**What the code reads.** `present_plane`'s two-line arm cuts the
corner's sector on the ray `m`, the sum of the two edges' unit
toward-face normals, into two pieces, the cones of `(a, m)` and
`(b, m)`, each presented as a convex corner with its own zero
(`Presentation::Pair`). `ray_inside` decides `m` strictly on the face
side of each edge at `SELF_INTERSECTION_CORNER`, a length times a sine
at a cell-scale lever, as the lean it replaces was; a spike or a slit
in the band refuses. The lean gate and the one quadrant are gone.
`departure` reads a face in two pieces piece by piece: a carrier parts
the pair only where it signs every piece of the other face; the planes
and the graph are asked of every piece pair; `one_shared_vertex`'s path
(PR 7329) asks every piece pair of the two presentations against the
vertex alone; the pair is certified only where all piece pairs are.

**Where the build departs from the design above.**

- **No near of its own per piece pair.** A separator through a shared
  edge lists the edge's ends, so it holds the corner of a piece off
  that edge, and the filleted corner (a plane's off-edge piece against
  a tangent cylinder whose side is on the edge) is parted by exactly
  that plane: `a_filleted_corners_ball_holds_against_a_face_at_its_pole`
  refused with the per-pair near. Where the plane through a shared edge
  is zero along the off-edge piece too (a straight corner), a valid
  plane/plane pair is closed by the carrier arm: the second edge would
  have to lie in both planes, that is on the shared line.
- **A shared edge's end is the off-edge piece's corner**, whether or not
  the partner's cell is near the vertex: the shared cell holds its
  edges' ends.
- **It needs PR 7329's margin.** On main's metering (a derivative times
  the rectangle's span) four of the acceptance rows refused: the flush
  union built at `(0.1, 0.2, 0.3)`, with sides 0.7, moved by
  `(1/3, 10.1, −7.7)`, and at the session box's wall (there `f64` held
  and `Interval` refused). Traced on the side-0.7 body: the L-shaped
  top face's cell lying just across a shared edge from the face is
  presented by the one-edge arm as a strip 2e-16 wide (at a non-dyadic
  placement its chart coordinate and the cell's bound are each an ulp
  wide), and against the straight wall's off-edge piece neither carrier
  signs it: the top's is zero along that piece's edge, the wall's reads
  the strip's departure times 2e-16. One quadrant, as the wall was
  before, is signed by the top's carrier. With 7329's sine at the
  faces' reach all nine body rows hold at both scalars. Two things
  tried on main's metering and dropped once stacked, each because its
  mutant survived: pieces overlapping past `m`, and a rule not to
  present a piece the other covers.
- **A self pair** (`certify_self`, the carrier arm off) cannot reach
  the pieces: a plane face's own identification at a vertex is its loop
  passing the vertex twice, four edges in the cell, which is not this
  arm. Stated, not pinned.

**Census, before and after** (`cp-measure-before` / `cp-measure-after`:
the gate harness `origin/embed/remeter-measure` merged onto
`embed/remeter` `7667c455e`, without and with this unit at
`84a3108e8`; taken before the landing branch's lune and pole came in).
Population: the census's 215 plane/plane departure tests.

| | before | after |
|---|---|---|
| pass with the gate asking | 28 | 77 |
| first refusal departure, vertex, plane/plane | 79 | 17 |
| first refusal departure, edge, plane/plane | 89 | 73 |
| `Crosses` | 13 | 32 |
| `Budget` | 6 | 11 |
| departure, cylinder/plane | 2 | 6 |

No test that passed before fails after. By the classes the stopgap's
probe gave the 179 tests it left refusing:

| class | tests | pass before | pass after | the rest, after |
|---|---|---|---|---|
| straight only to rounding | 32 | 5 | 20 | 8 `Crosses`, 3 `Budget`, 1 edge |
| reflex exits only | 78 | 21 | 29 | 20 edge, 7 vertex, 9 `Crosses`, 7 `Budget`, 4 cylinder/plane |
| no two-line exit | 66 | 2 | 4 | 52 edge, 9 vertex, 1 `Budget` |
| exact leans, another condition | 3 | 0 | 2 | 1 vertex |

So the pieces close the straight corner (15 of the 27 left, the rest
reaching a recorded contact or a budget) and little of the reflex one:
21 of those 78 already pass on the base without this unit, and the
pieces add 8. That agrees with `embed/iso-end`'s reading that a reflex
pair is closed through its partner. None of the 8 is closed by the
carrier arm alone: with the piece-pair ladder and the vertex-alone path
switched off at every pair holding a corner that is not strictly convex
(a measurement-only switch on the measuring tree, the carrier arm still
asked of every piece) 19 of the 78 pass, none of them among the 8.
The 72 that still refuse at an edge exit elsewhere (a thin face, more
than two edges in the cell).

**Pins.** Unit, `injective::presentation_rows`:
`a_two_line_corner_is_two_pieces_that_cover_its_sector`,
`a_spike_or_a_slit_within_the_band_is_not_cut`,
`faces_sharing_a_vertex_alone_are_parted_piece_by_piece`,
`faces_sharing_an_edge_of_the_corner_are_parted_or_refused`, and
`a_planes_reflex_corner_of_two_lines_is_not_its_wedge` re-pinned. Body,
each at both scalars: the flush union on the dyadic grid, built at
`(0.1, 0.2, 0.3)`, with sides 0.7, moved by `(1/3, 10.1, −7.7)`, a
hundred units out, at the session box's wall, turned about a generic
axis, mirrored by the mirror door across a generic plane, its mirror
image built and turned, and with its profiles turned in their sketch
plane; a flap through a run at its vertex stays `Violated`. One row is
tied to an ε: at 1e-12 the `Interval` union at the wall does not build
(`work/contacthold/a-flush-union-at-the-session-boxs-wall-does-not-build-at-interval-at-the-tightest-eps.md`),
and the row asserts the `f64` body there.

**Cost, on the build** (the same two trees and 215 tests, the gate's
receipts summed per test). Over all 215: gate calls 28,271 to 39,374,
discharged cell pairs 9,019,243 to 12,274,408 (1.36), mostly tests
that now get further (64 make more gate calls than before). Over the
145 tests whose gate calls are the same: discharged 3,877,010 to
4,000,416 (1.03), per-test median 1.00, least 0.41, most 1.44. The
design's worst case, `review_f7_pole_r1_probes::p3_inset_coplanar_patch_still_refuses`
(estimated 2.49 times the asks), discharges 1,531 pairs before and 635
after: the estimate counted piece pairs per call under a build where
its corners still split to the depth floor. The suite file and unit
set, the 283 tests both trees have: 204.6 s on `8d34076a9e`, 205.3 s
here.

**The tour walk** (85 stops, `8d34076a9e` against the unit merged with
it): no verdict moves, 82 hold on both and the three benches read
`Crosses` on both; 26 receipts move, total splits 304,934 to 301,998.

**Mutants** (each one exact edit, over the 199 `injective::` unit tests
and the 99 tests of the suite file; the edits are in the PR's body):
the shared-edge-end rule, any-piece-pair under the planes and graph,
the carrier arm on any piece, the vertex-alone path on any piece pair,
the edge-span check and the off-cone piece each redden a unit row; the
off-cone piece also reddens three body rows. The ray's decision
removed survives at the default ε and reddens
`a_spike_or_a_slit_within_the_band_is_not_cut` at ε = 1e-6: at the
default band a spike thin enough to be in the band is too thin for the
ray's direction to be certified, and the division that places the cell
refuses first.

**Beside `embed/strip`** (PR 7334, merged under the unit at
`7f85375799`). `present_plane` chooses its reader by the edges the cell
holds: one edge is strip's `along`; two edges with a common vertex are
the pieces, whatever the corner's angle; three edges are a strip's end,
read by strip's convex `corner` with its cap, which is now always given
one (the uncapped branch was the lean gate and quadrant this unit
replaces). A thin face at a two-line vertex is the pieces' while the
cell holds only the two edges and strip's once it holds the end's third
edge. Strip's random coverage row
(`a_presented_end_holds_the_faces_part_at_random`) now asks that each
face point lie in some piece and passes over two-edge cells too;
`a_poisoned_line_direction_is_refused_by_every_reader`'s sound control
is two pieces. The endpoint census's entry for `injective.rs` is 215
reads, 102 asking (the landing tip's 211 / 98 and the pieces' five,
less the uncapped corner's one). The census through the gate, the cost
and the tour walk above were taken before this merge and not re-taken.
