---
id: main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps
kind: issue
title: Main red: eighteen sweep analytic rows (rulings, cylinder_sphere_frame, torus_doors_join, …) are red on main at the default ε
status: open
opened: 2026-10-31
priority: P0
cost: M
---

Found by SPLICE's curved-crossing lane (PR 6136) while validating locally
on 2026-10-31. The lane re-ran the failing rows on bare `origin/main`,
and all 18 are red there too, so they are main's, not 6136's. The run
used the sweep slow set at the default ε. Not yet bisected.

## The rows (sweep)

- `a_ruling_lying_on_a_wall` ×2
- `a_ruling_through_a_box_corner_edge` ×2
- `a_pierce_held_in_a_curved_face` ×2
- `bisector_nodes::the_near_node_poses_at_the_witness_band`
- `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_*` ×6
- `torus_doors_join` ×3
- `conic_edge_curved_face::a_rim_crossing_reaches_the_join`
- `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`

## What closing it takes

Reproduce each row on current main and bisect it. The candidates are the
merges of 10-30 and 10-31, which include SECTARMS's cylinder × cylinder
unit (PR 6139), the touch-or-tangency split and VSEAM PR 6129. Then fix
each row or route it to the program whose merge caused it. Some of these
may already be covered by an existing row:
`charttail/main-red-ringed-curved-doors-reds-four-sweep-rows` lists
`bisector_nodes` at 1e-12. If one is, add this evidence to that row
rather than duplicating it.


## PR 6139 cleared (2026-10-31, SECTARMS)

SECTARMS ran the 18 rows at the default ε, with `--run-ignored all --no-fail-fast` in its own clone, at three commits:
- `81707fbe`, the first parent of 6139's merge (main before 6139);
- `f2f34e16`, 6139's merge;
- `cf488e3`, SECTARMS's stage 2 (PR 6151).

17 rows are red at all three, with identical fail sets. The panic texts before and after 6139 hash the same, so each row fails at the same site with the same message:
- **`a_pierce_held_in_a_curved_face`, `a_ruling_lying_on_a_wall`, `a_ruling_through_a_box_corner_edge`:** `Err(EdgeInFace { …, gap: CurvedFace })`, and on one `a_pierce_held` row a tier-3′ `FaceThroughEdgeWedge`. Seen at `a_pierce_held_in_a_curved_face.rs:447`, `a_ruling_lying_on_a_wall.rs:232` and `a_ruling_through_a_box_corner_edge.rs:141`.
- **`cylinder_sphere_frame` ×6 and `conic_edge_curved_face`:** `ResultInvalid { VolumeUncomputable … }`.
- **`bisector_nodes::the_near_node_poses_at_the_witness_band`:** `Escalated { Coincidence(EdgeOnCurvedFace, Moot) }`.
- **`torus_doors_join` ×3:** `assert_eq` mismatches at `torus_doors_join.rs:136`, `:152` and `:176`.

The 18th row, `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`, was still running after 22 minutes at `81707fbe` and was killed. So whatever makes it red already predates 6139.

**6139 leaves the suspect list.** The earliest commit checked is `81707fbe`, so bisection should start before it. `charttail/main-red-ringed-curved-doors-reds-four-sweep-rows` overlaps only in the `bisector_nodes` file, and on a different row (`slanted_bores_…` at 1e-12).

## 2026-10-31 (SECT, lane `sect-onesix`)

Measured on main `b719d39ad1` at opt-level 2. Two of these rows also
read the same at 1e-6 and 1e-12 as at the default ε:

- `torus_doors_join::a_square_rod_on_the_outer_equator_stops_at_the_join`
  stops `Projected`, where the row wants `Join { closed: false }`.
  `a_rod_across_the_inner_bound_…` stops `Projected` and
  `a_rod_off_the_axis_line_…` stops `ConicTrim`, at every ε.
- `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`:
  for β 0.5 the five ops stop `ring volume`, `quadrature volume`,
  `quadrature volume`, `ring volume`, `quadrature volume`. For β 0.6 and
  0.7 all five stop `quadrature volume`. `OTHER_SPINS_STOPS` wants
  `spline join` on ∪ and on c ∖ h.

Until SECT's PR on
`sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot`
lands, both rows are pinned at 1e-6 to an earlier stop, the germ
section's `Winds(Unseparated)`. That PR removes the two pins. With the
pins gone, at 1e-6 both rows stop exactly as listed above, so all three
ε now share this one red.

## A topo fuzzer counterexample on main at 1e-6 (RESTCURVE, PR 6163, 2026-11-01)

Not a sweep row, filed here as main-red evidence on the orchestrator's
direction rather than as a new row. `topo`
`chord_join::wall_ring_rows::a_spline_bounded_wall_island_re_homes_its_bystanders_as_the_ball_says`
fails at ε 1e-6 under `CAD_FUZZ_SEED=0x9cb9baff9ceb8aeb CAD_FUZZ_EFFORT=1`:
"asked 12, escalated 0: centre `(0.5, 0, 0)`, cuts `[0.3149597220281418]`:
`RingHomingAmbiguous { ring: LoopKey(5v1) }`". It reproduces on main at
`336465c3` with the same seed, in a clean worktree, so it is main's and
not the region arm's. The test draws its seed per run, so it is red only
on runs that draw this one. By the discipline's fuzzer rule, the
counterexample is owed a pinned deterministic row beside its fix.

## Two blend-wording rows red on main since PR 6200 (RESTCURVE, PR 6163, 2026-11-01)

Not sweep-analytic rows; filed here as main-red evidence rather than as a
new row. Both fail on bare `origin/main` at `a95036de` at the default ε,
in a clean worktree, and both read text that PR 6200
(`chrome/blend-recourse-words`, merge `c43600e2`) re-worded in
`crates/sweep/src/blend/mod.rs`:

- `editor-core` `refusal_concision_chains::every_node_refusal_renders_within_the_budget`
  (`crates/editor-core/tests/refusal_concision_chains.rs:711`):
  `Blend/UnsupportedRunOut` renders 90 words and
  `Blend/UnsupportedCorner(mixed)` 95, both over the 75-word budget, the
  new corner recourse carrying most of it.
- `sweep` `m5_pr12_die_body::a_subset_of_the_edges_refuses_at_the_assembly_front_door`
  (`crates/sweep/tests/m5_pr12_die_body.rs:185`): "the refusal names the
  missing front door" no longer matches the re-worded curved-end-face
  refusal ("a straight blend would have to stop against a curved face,
  which is not supported yet; …").

Neither test was touched by PR 6200, and both are outside its diff's crate
tests, so its own CI did not run them red.
