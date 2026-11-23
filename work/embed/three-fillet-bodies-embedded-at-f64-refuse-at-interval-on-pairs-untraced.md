---
id: three-fillet-bodies-embedded-at-f64-refuse-at-interval-on-pairs-untraced
kind: issue
title: Fillet bodies whose support planes now hold at their tangent joints refuse at Interval, and one at f64, on other pairs: a wall plane against a corner patch at a vertex alone, a strip against a patch across their meridian, two walls across a tangent seam, and one depth budget, none traced
status: open
opened: 2026-11-22
priority: P0
cost: M
refs: [a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation]
---

## Finding

From lane `embed/tangent-corner` (2026-11-22). With a support plane's
tangent joints read
(`a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation`),
the fillet bodies of that row get past the plane against the corner
patch across its arc, and some then refuse on a pair that reading does
not touch. None of these is traced to its exit.

**Three bodies pinned in `crates/sweep/tests/self_intersection_proves_injectivity.rs`**
(`a_support_face_c1_through_a_vertex_at_interval`), each `Embedded` at
`f64` in three poses and refused at `Interval` as built, through
`topo::clearance::embedded` at the default depth and 65,536 pairs:

| body | at `Interval` |
|---|---|
| the inset corner, three edges filleted at 0.1 | `Uncertified { Departure }`, a wall plane against the torus patch at a vertex they share alone |
| a 2 × 1.2 × 0.5 plate of corner radius 0.3, every edge filleted at 0.1 | `Uncertified { Departure }`, a cylinder strip against a torus patch across an edge (their meridian) |
| the four-arc barrel, every edge filleted at 0.05 | `Uncertified { Departure }`, cylinder against cylinder across an edge (two walls at their tangent seam) |

In each, every support plane against every patch it shares an arc
with holds at `Interval`, asked alone.

That table is main `0c1eb97ad`. With main `02a9a2df4` merged in, the
inset corner at `Interval` reads `Budget(Depth { max_cell_depth: 40 })`
instead, at the default ε and at 1e−6; the other two read as above.
Which pair the door names first moves with the build, so the suite row
pins only that the three are refused, and prints what each reads. At
ε = 1e−12 the three do not fillet at `Interval` at all (the band's own
escalation), and the row says so and skips them.

The inset corner's pair, read with the gate's cell trace on
`band_mixed_corner_reach::the_inset_witness_fillets_or_escalates_at_every_eps_row`
(its `Interval` body): plane `8v3` against torus `15v3` at vertex
`23v1`, depth 40. The torus is presented with the vertex as its corner
point in every cell; the plane is presented, the vertex its corner
point too, in cells down to `0.0625` wide, and is `Local` in every
smaller one traced, the last
`u ∈ (0.39956, 0.4000000000000038)`, `v ∈ (0.22500, 0.22559)`. Which
of `present_plane`'s clauses returns is not read.

**The same shapes in the census tests of the row, gate asking**
(`Interval` asks reached once the `f64` ask before them holds):
- cylinder against torus across an edge:
  `band_closed_mixed_chain_shapes::arcs_flanking_a_short_flat_fillet_whole`
  (`3v1 × 19v1`, `49v1`),
  `band_mixed_rim_shared_supports::a_rounded_plate_with_a_round_hole_fillets_every_edge_in_one_call`
  and `holes_beside_a_corner_build_in_one_call_as_in_sequence`
  (`3v3 × 21v1`, `55v1`),
  `rectangular_holes_ending_at_a_corners_window_build_at_pappus`
  (`3v3 × 23v1`, `61v1`);
- cylinder against cylinder: `a_four_arc_barrel_fillets_whole`
  (`3v1 × 4v1`, edge `14v1`), and at a vertex alone
  `a_cross_drilled_plates_top_rim_fillets_with_the_bore_on_its_mates`
  (`3v1 × 28v1`, `28v1`);
- at `f64`, `band_mixed_corner_reach::the_bracket_builds_at_the_witness_sizes_and_up_to_its_real_limit`
  refuses `Budget(Depth { max_cell_depth: 40 })`, where before the
  change it refused plane against torus across the arc `39v1`.

`the-filleted-drum-at-interval-refuses-on-a-straddling-rate-at-its-tangent-seam`
may be the barrel's cause; that is not checked.

## What is needed

Each traced to its cell and exit, then re-homed on the row that owns
its cause or given one.

