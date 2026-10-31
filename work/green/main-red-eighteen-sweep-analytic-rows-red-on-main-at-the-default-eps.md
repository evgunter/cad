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
