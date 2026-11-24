---
id: a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation
kind: issue
title: A plane face's corner where a line continues its shared arc tangentially is read by no corner presentation, so every fillet band whose rim arc meets its straight foot at a tangent joint refuses Departure at the joint
status: closed
opened: 2026-11-21
priority: P0
cost: M
refs: [departure-reads-no-curved-corner-off-its-shared-edge]
pr: 7320
branch: embed/tangent-corner
closed: 2026-11-24
---


## Finding

From EMBED's per-solid census trace (lane `embed/trace-cone-torus`,
2026-11-21), main `c2a0c41f40` with the gate and its trace switches,
plus prints at each of `graph_read`'s exits
(`embed/trace-cone-torus-measure`).

A fillet band's corner patch is a torus face whose foot on the support
plane is an arc, joined at each end to a cylinder fillet's straight
foot. The pair is the plane against the torus across the arc; it splits
to `Depth { max_cell_depth: 40 }` and refuses `Uncertified { Departure }`.

**The cell.** In `band_mixed_corner::an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form`
the last pair is the plane `1v1` at `u ∈ (−0.10059, −0.09961)`,
`v ∈ (0.09961, 0.10156)` against the torus `15v3` at its parallel
`v = 3π/2`. The plane cell holds the vertex `21v1` where the shared arc
`33v1` ends and the next foot begins. The torus is presented by its
parallel (`Zero::Side(YHi)`); the plane is `Local`.

**The exit.** `present_curved` reads the arc through the vertex
(`through_of`) and `graph_read` fails in both columns:
- column `V` (the arc as `u` over `v`): `monotone_run` fails. The arc's
  `v` over the cell is `(0.099759, 0.1)`, and `0.1` is its value at the
  vertex: the arc's `v` turns at the vertex, so it is no graph over `v`;
- column `U` (as `v` over `u`): the run is chosen and is monotone, and
  `graph_read` then returned `None` in its corner block (after `apart`
  and `alone`, which do not fire), at `leaves_graph`'s rate: measured
  below, 104 exits of 104, the vertex premise never failing.

So `present_curved` returned `Local` at every depth, and the corner
was in every cell along the arc's end. (Read since: "Read: one run
through the vertex", below.)

The same cell shape in two more traced rows of the family,
`band_mixed_rim_shared_supports::a_cross_drilled_plates_top_rim_fillets_with_the_bore_on_its_mates`
and `band_closed_mixed_chain_shapes::a_creased_corner_refuses_where_definite_and_builds_below_the_band`:
the arc at its turning point at the vertex in one column, and the other
column's run lost in the corner block.

**The cause, checked** (lane `embed/tangent-corner`, below). The arc's
`v` turning at the vertex is read off the traced reaches. The next
edge leaves the vertex along the arc's tangent, to rounding, and
`leaves_graph`'s rate of the line off the continued arc straddles zero
in every cell, which no first-order reading signs.

`departure-reads-no-curved-corner-off-its-shared-edge` covers corners
of two different conics and a plane's line edge meeting an arc; this is
a curve read through a corner whose other edge continues it at first
order.

## Count

24 tests, all `band_*` and `blend_recourse_followability` fillets with
no Boolean record (departure at an edge, plane/torus, 23 f64 and one
`Interval`). Three traced. Not counted: the two `lamina_annulus` rows
of that bucket, whose plane cell is `Local` at `beside_curve`'s last arm
(two curved walls, no corner read), untraced.

Not this cause, measured below: the `lamina_annulus` rows and
`a_split_through_a_wrap_edges_vertex`'s (a plane cut tangent to a
hole's rim at a vertex; `present_plane`'s two-line corner lean check).
No verdict of theirs moves with the tangent joint read.

The bodies are valid: fillets built at their closed forms.

**Cheapest reproducer:** `sweep::all band_mixed_corner::an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form`
(0.35 s).

## What is needed

A corner reading for a curve continued at first order by its next edge:
the next edge's departure from the curve read at second order (its
curvature against the arc's), or the two edges read as one `C¹` run
through the vertex.

## Read: one run through the vertex (EMBED, lane embed/tangent-corner, 2026-11-22)

**Measured first** (main `0c1eb97ad` merged with `embed/gate-measure`
and `embed/trace-cone-torus-measure`, f64, prints at each clause of
`graph_read`'s corner block and inside `leaves_graph`, uncommitted;
`band_mixed_corner::an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form`).
- 104 corner-block entries, at both ends of the arc `33v1` (vertices
  `21v1` and `22v1`). In every one the vertex premise holds and
  `leaves_graph` returns at its rate, `Indeterminate` at
  `self_intersection_corner_departs`: the enclosure straddles zero at
  every cell size, from `(−0.137, 0.206)` at the root to
  `(−1.9e−6, 2.9e−6)` at the floor.
- The joint is tangent to rounding. At `22v1` the arc's chart tangent
  is `(2.9e−17, −0.2)` and the line `35v1` leaves along `(0, −0.9)`.
- The plate's top lies outside the arc's circle there (`above` at
  higher `u` than the arc, whose centre is at lower `u`), so the line
  stands on the face's side of the circle continued, at second order.
  On a rounded outline the face is inside the circle and the line is on
  the other side of its continuation: the face itself crosses the
  continued circle, and no reading through the circle bounds it.

**What landed** (`crates/topo/src/injective.rs`): the two edges read as
one run.
- **The reading** (`present_corner`, "One run through the vertex";
  `graph_read`, `line_onward`, `curve_onward`). Where the corner's other
  edge is not decided to leave onto the read curve's face side
  (`leaving`: undecided, or decided reflex), the curve up to the vertex
  and the other edge from it are read as one graph over the column
  axis. The other edge must run on across the columns the way the curve
  came to the vertex, past their far end; `R` holds its reach; nothing
  else may be in reach. No angle between the edges is read, so the
  joint needs no margin: it is the same reading tangent, a few ε off
  tangent, or reflex.
- **Why not the second-order reading.** It exists only where the next
  edge lies on the face's side of the curve continued (a concave arc).
  On a convex one the face crosses the continuation. It would also
  need a decision that the line does not dip back across the circle,
  which a joint tangent only within the band does.
- **From either edge.** A plane's line edge whose conic is not decided
  onto its face side (`present_plane_corner` failing) is read by the
  conic, the same run, its part on the shared cell the line's
  (`Corner::run`).
- **A curved other edge is not read** (an arc into an arc of another
  radius). The first head read it by a ladder of its own, which the
  review showed unsound and which was removed:
  `a-corner-of-two-curves-is-read-as-no-run`.
- **Its zero** is the run, `past` the vertex along the edge that is not
  shared (`Curved::run`, `RunOn`). It is read only where `past_read`
  admits a past curve, and only under a graph separator whose graph
  runs on along that edge (`graph_run`, `On::run`): its slopes are the
  hull over both edges, and its own edge stops at the vertex's
  parameter. Not under a plane, and not in the carrier arm.
- **The graph's own edge ends at the vertex, by key.** A partner's
  zero side clipped at a line's enclosure reaches a rounding past the
  vertex's parameter (measured `4.4e−16` on
  `rounded_polygons_fillet_whole`, the torus side `(−4.44e−16, 0.0041)`,
  the vertex at 0). That is no stretch of the edge: the side's edge
  ends at that vertex, and the clip is the line whose own edge ends
  there, so the side is read to the vertex (`Along::stops`, recorded by
  `present_chart` from the clip's edges; `Along::ending`). No length is
  compared and no concession read. A side that names no such vertex,
  or another vertex, and passes the run's vertex by any distance gets
  no graph. `edge_piece` also returned the edge's own reach a rounding
  past its span (`t0 + width` is not `t1`); it now ends at the span's
  end. The first head instead admitted the stretch where it was decided
  `Zero` at the vertex concession's site, which the orchestrator ruled
  out and the review showed read `1e−10` past the vertex on no key.
- **Typing.** A reflex corner whose other edge is no graph with the
  curve over a column (it does not advance that way) stays
  `Structural`; one that is, but is not read in this cell, is `Local`.
  A column where the run is no graph no longer stops the other column
  from being tried. A run is not `stuck`.

**Measured after** (the same harness; before is main `0c1eb97ad` with
the gate; 35 tests: this row's 24, `lamina_annulus`'s four and
`a_split_through_a_wrap_edges_vertex`'s seven):

| | before | after |
|---|---|---|
| of the 24, refusing plane × torus across an arc | 24 | 0 |
| of the 24, passing with the gate asking | 0 | 11 |
| of the 24, refusing plane × cylinder across the straight foot | 0 | 5 |
| of the 24, refusing elsewhere (below) | 0 | 8 |
| the 11 `lamina_annulus` and `a_split…` tests | 11 fail | 11 fail, every payload the same |

- **The 11 that pass**: `band_closed_mixed_chain::a_rounded_pockets_floor…`,
  `band_closed_mixed_chain_shapes::{a_creased_corner…, a_hole_beside_the_rim…, holes_at_an_arcs_wedge…}`,
  `band_mixed_corner::{an_inset_corner…, the_bracket_fillets_all_round…}`,
  `band_mixed_corner_names::the_brackets_mixed_corners…` (editor-core),
  `band_mixed_corner_reach::{an_island_entering_the_inset_corners_patch…, an_island_entering_the_inset_patch_at_a_non_right_plan_angle…}`,
  `band_mixed_rim_shared_supports::{a_bore_reaching_a_mates_strip…, a_hole_reaching_a_links_strip…}`.
- **The 5 across the straight foot** are not this cause: the strip's
  section arc is stored a rounding off a parallel on a turned frame, so
  the cylinder's corner is read by its ruling run on past the vertex
  (`work/pcert/a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64.md`).
- **The 8 elsewhere** get past their `f64` ask and refuse a later one,
  six at `Interval` and one at `f64` on the depth budget, plus this
  row's own `Interval` test at a vertex alone
  (`three-fillet-bodies-embedded-at-f64-refuse-at-interval-on-pairs-untraced`).
- **The `lamina_annulus` and `a_split…` rows are not this cause**: no
  payload of theirs moves.

- **The injectivity suite** (`self_intersection_proves_injectivity`,
  no gate): 52 pass at the default ε, 1e−6 and 1e−12, main's 49 and
  three new rows. No existing row's pin moves.
- **The tour walk**, 85 stops (main's `injective.rs` against this one
  in one checkout, release): identical. 82 hold, the three bench
  assemblies cross at their recorded contacts, and no body's
  candidates, discharged, refused or splits move.
- **The endpoint census**: `injective.rs` 177 → 194 reads, blessed
  with their disposition. Three other files' counts were already off
  on main (`pcurve_cache/speed.rs`, `step-import`'s `on_built.rs` and
  `recognize_curve.rs`), left for their owners.

The measurements above the tour were taken at `165a534b1` on main
`0c1eb97ad`; what landed after is rows, docs and one comparison that
reads a parameter that is no number as past the vertex.

**Pinned** (`crates/topo/src/injective.rs`, `presentation_rows`; the
body rows in `crates/sweep/tests/self_intersection_proves_injectivity.rs`):
the tangent joint as one run from either edge; a reflex corner that is
a graph, as a run; a third edge in reach; the graph run on along the
other edge, its slopes, and the three places a run reads nothing; a
fold through a tangent joint; three bodies `Embedded` at `f64` as
built, turned and a hundred units out (the face outside the arc's
circle and inside it), the plate drawn turned as the finding it is,
and a crossing copy; and the review's rows (a curved next edge that
comes back into `R`, a hairpin line, a graph two-valued along `t`, a
run under its conic's plane, a keyless side past the vertex).

### Review fix pass (2026-11-22)

Measured again with main `4d3402ecb1` merged, the gate asking, this
row's 24 tests and the 42 plane × cylinder edge tests of the modules
`a-plane-corner-where-a-line-runs-on-tangent-into-an-arc-reads-no-leaving`
names (its 37 are among them; the row lists modules, not tests):

| | this row's 24 | the 42 |
|---|---|---|
| passing, before | 0 | 0 |
| passing, with the end read by key | 11 | 22 |
| refusing at `f64`, plane × cylinder across a line | 4 | 7 |
| refusing at `f64`, plane × torus across an arc | 0 | 1 |
| refusing at `f64`, elsewhere (depth or pair budget, a vertex alone) | 2 | 5 |
| refusing at `Interval` | 7 | 7 |

Those figures have the curved next edge still read. **Measured again
on the frozen head `bdd319229`, after its removal** (2026-11-23, the
same sets): **11 of the 24 and 21 of the 42 pass.** Two tests moved,
both an arc into an arc, both now `Uncertified { Departure }` plane ×
torus across an arc at `f64`
(`work/embed/a-corner-of-two-curves-is-read-as-no-run.md`):
`a_four_arc_barrel_fillets_whole`, of the 24, which was failing
already (it got past `f64` and refused at `Interval`), so the 24's
count stays 11 with one more across an arc at `f64` and one fewer at
`Interval`; and `band_open_mixed_chain_shapes::a_compound_corner_of_two_arcs_builds`,
of the 42, which passed with the ladder.

- The 11 at `f64` across a line are outlines built off the axes
  (`work/pcert/a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64.md`),
  one traced.
- The one of the 42 still across an arc
  (`band_mixed_corner::a_pair_edge_too_short_for_its_two_ends_feet_refuses_at_them`,
  `2v1 × 17v3`, edge `32v1`) is not traced.
- By key closes what the concession closed on this row (11).

**What this row leaves:**
- a corner of this kind at a vertex the pair shares alone (the run is
  read only where one of its edges is shared);
- a curved other edge (`a-corner-of-two-curves-is-read-as-no-run`);
- a spiric wall's corner (`present_wall` keeps `leaves_graph`);
- a coordinate line of a cylinder's or a torus's chart whose curve
  leaves it tangentially, read from the line's side
  (`present_line_corner`; the run is read from a plane's line only).

Only the curved other edge has a measured body (the barrel). Two lines collinear at a vertex are
`present_plane`'s, not a curve reading's.

## Census tests the cylinder fix leaves at present_curved and graph (EMBED lane `embed/iso-end`, 2026-11-22)

In these 9 the cylinder's pair is refused with the partner, a plane face, unpresented at `present_curved` (no column's graph reads, `graph_read`'s `Local`). Six are the tangent-boss documents (`intent_s6_*`, `coincidence_door`'s boss), built tangent by construction. In the last three (a rod glued in its bore, a rod at a box edge, the ring lane) the exit is the graph's; whether they are tangent was not checked.

Measured on `embed/iso-end-measure2` (`origin/embed/gate-measure` @ `ebb2e6f8e` plus `embed/iso-end`'s diff), where each refusal is the same pair, test by test, as on the trace run that named the exits (`embed/iso-end-measure` @ `030c75763`, `CV_TRACE`).

- `editor-core::all coincidence_door::a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency`
- `editor-core::all intent_s6_d_tangency::a_boss_traced_off_the_walls_axis_is_tangent_by_construction`
- `editor-core::all intent_s6_d_tangency::a_boss_traced_on_one_cylinder_is_unproven_against_its_image`
- `editor-core::all intent_s6_d_tangency::a_boss_traced_on_the_walls_other_half_proves`
- `editor-core::all intent_s6_d_tangency::a_heading_typed_along_the_tangent_is_unproven`
- `editor-core::all intent_s6_d_tangency::a_frame_slid_spun_or_flipped_along_the_axis_still_proves`
- `sweep::all bisector_nodes::a_rod_glued_in_its_bore_answers_alike_in_either_order`
- `sweep::all curved_coincident_fates::a_rod_at_a_box_edge_borders_alike_in_either_order`
- `sweep::all verbs_germarms_interval::the_ring_lane_builds_at_the_certified_scalar`
